use lru::LruCache;

use crate::component::Component;
use crate::DataReader::json_reader;
use crate::DataReader::json_writer::component_to_json_file;
use crate::DataReader::xml_parser::parse_xml_from_file;
use crate::ModelObjects::queries::Query;
use crate::ModelObjects::system_declarations::SystemDeclarations;
use crate::System::input_enabler;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

type ComponentsMap = HashMap<String, Component>;

struct ComponentTuple {
    components_hash: u32,
    components_map: Arc<ComponentsMap>,
}

/// A struct used for caching the models.
#[derive(Debug, Clone)]
pub struct ModelCache {
    // TODO: A concurrent lru may be faster to use and cause less prone to lock contention.
    cache: Arc<Mutex<LruCache<i32, ComponentTuple>>>,
}

impl Default for ModelCache {
    fn default() -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::<i32, ComponentTuple>::new(
                NonZeroUsize::new(100).unwrap(),
            ))),
        }
    }
}

impl ModelCache {
    /// A Method that returns the model from the cache.
    ///
    /// # Arguments
    ///
    /// * `components_hash` - A hash of the components
    pub fn get_model(&self, user_id: i32, components_hash: u32) -> Option<ComponentContainer> {
        let mut cache = self.cache.lock().unwrap();

        let components = cache.get(&user_id);

        components.and_then(|component_pair| {
            if component_pair.components_hash == components_hash {
                Some(ComponentContainer::new(Arc::clone(
                    &component_pair.components_map,
                )))
            } else {
                None
            }
        })
    }

    /// A method that inserts a new model into the cache.
    ///
    /// # Arguments
    ///
    /// * `components_hash` - A hash of the components
    /// * `container_components` - The `ComponentContainer's` loaded components (aka Model) to be cached.
    pub fn insert_model(
        &mut self,
        user_id: i32,
        components_hash: u32,
        container_components: Arc<ComponentsMap>,
    ) -> ComponentContainer {
        self.cache.lock().unwrap().put(
            user_id,
            ComponentTuple {
                components_hash,
                components_map: Arc::clone(&container_components),
            },
        );

        ComponentContainer::new(container_components)
    }
}

pub trait ComponentLoader {
    fn get_component(&mut self, component_name: &str) -> &Component;
    fn save_component(&mut self, component: Component);
}

#[derive(Debug, Default, Clone)]
pub struct ComponentContainer {
    pub loaded_components: Arc<ComponentsMap>,
}

impl ComponentLoader for ComponentContainer {
    fn get_component(&mut self, component_name: &str) -> &Component {
        if let Some(component) = self.loaded_components.get(component_name) {
            component
        } else {
            panic!("The component '{}' could not be retrieved", component_name);
        }
    }
    fn save_component(&mut self, _component: Component) {
        //Intentionally left blank (no-op func)
    }
}

impl ComponentContainer {
    pub fn new(map: Arc<ComponentsMap>) -> Self {
        ComponentContainer {
            loaded_components: map,
        }
    }
}

pub trait ProjectLoader: ComponentLoader {
    fn get_declarations(&self) -> &SystemDeclarations;
    fn get_queries(&self) -> &Vec<Query>;
    fn get_project_path(&self) -> &str;
    fn to_comp_loader(self: Box<Self>) -> Box<dyn ComponentLoader>;
}

pub struct JsonProjectLoader {
    project_path: String,
    loaded_components: ComponentsMap,
    system_declarations: SystemDeclarations,
    queries: Vec<Query>,
    should_reduce_clocks: bool,
}

impl ComponentLoader for JsonProjectLoader {
    fn get_component(&mut self, component_name: &str) -> &Component {
        if !self.is_component_loaded(component_name) {
            self.load_component(component_name);
        }

        if let Some(component) = self.loaded_components.get(component_name) {
            component
        } else {
            panic!("The component '{}' could not be retrieved", component_name);
        }
    }

    fn save_component(&mut self, component: Component) {
        component_to_json_file(&self.project_path, &component);
        self.loaded_components
            .insert(component.get_name().clone(), component);
    }
}

impl ProjectLoader for JsonProjectLoader {
    fn get_declarations(&self) -> &SystemDeclarations {
        &self.system_declarations
    }

    fn get_queries(&self) -> &Vec<Query> {
        &self.queries
    }

    fn get_project_path(&self) -> &str {
        &self.project_path
    }

    fn to_comp_loader(self: Box<Self>) -> Box<dyn ComponentLoader> {
        self
    }
}

impl JsonProjectLoader {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(project_path: String, should_reduce_clocks: bool) -> Box<dyn ProjectLoader> {
        let system_declarations = json_reader::read_system_declarations(&project_path).unwrap();
        let queries = json_reader::read_queries(&project_path).unwrap();

        Box::new(JsonProjectLoader {
            project_path,
            loaded_components: HashMap::new(),
            system_declarations,
            queries,
            should_reduce_clocks,
        })
    }

    fn load_component(&mut self, component_name: &str) {
        let mut component = json_reader::read_json_component(&self.project_path, component_name);
        if self.should_reduce_clocks {
            component.reduce_clocks(component.find_redundant_clocks());
        }

        component.create_edge_io_split();

        let opt_inputs = self
            .get_declarations()
            .get_component_inputs(component.get_name());
        if let Some(inputs) = opt_inputs {
            input_enabler::make_input_enabled(&mut component, inputs);
        }

        self.loaded_components
            .insert(String::from(component_name), component);
    }

    fn is_component_loaded(&self, component_name: &str) -> bool {
        self.loaded_components.contains_key(component_name)
    }
}

pub struct XmlProjectLoader {
    project_path: String,
    loaded_components: ComponentsMap,
    system_declarations: SystemDeclarations,
    queries: Vec<Query>,
}

impl ComponentLoader for XmlProjectLoader {
    fn get_component(&mut self, component_name: &str) -> &Component {
        if let Some(component) = self.loaded_components.get(component_name) {
            component
        } else {
            panic!("The component '{}' could not be retrieved", component_name);
        }
    }

    fn save_component(&mut self, _: Component) {
        panic!("Saving components is not supported for XML projects")
    }
}

impl ProjectLoader for XmlProjectLoader {
    fn get_declarations(&self) -> &SystemDeclarations {
        &self.system_declarations
    }

    fn get_queries(&self) -> &Vec<Query> {
        &self.queries
    }

    fn get_project_path(&self) -> &str {
        &self.project_path
    }

    fn to_comp_loader(self: Box<Self>) -> Box<dyn ComponentLoader> {
        self
    }
}

impl XmlProjectLoader {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(project_path: String, should_reduce_clocks: bool) -> Box<dyn ProjectLoader> {
        let (comps, system_declarations, queries) = parse_xml_from_file(&project_path);

        let mut map = HashMap::<String, Component>::new();
        for mut component in comps {
            if should_reduce_clocks {
                component.reduce_clocks(component.find_redundant_clocks());
            }
            component.create_edge_io_split();

            let opt_inputs = system_declarations.get_component_inputs(component.get_name());
            if let Some(opt_inputs) = opt_inputs {
                input_enabler::make_input_enabled(&mut component, opt_inputs);
            }

            let name = String::from(component.get_name());
            map.insert(name, component);
        }

        Box::new(XmlProjectLoader {
            project_path,
            loaded_components: map,
            system_declarations,
            queries,
        })
    }
}
