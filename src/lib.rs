extern crate libc;
extern crate lv2;

enum LilvPlugin {}
enum LilvPluginClass {}
enum LilvPort {}
enum LilvScalePoint {}
enum LilvUI {}
enum LilvNode {}
enum LilvWorld {}

enum LilvIter {}
enum LilvPluginClasses {}
enum LilvPlugins {}
enum LilvScalePoints {}
enum LilvUIs {}
enum LilvNodes {}

#[repr(C)]
struct LilvInstance {
    lv2_descriptor: *const lv2::LV2Descriptor,
    lv2_handle: lv2::LV2Handle,
    pimpl: *mut libc::c_void
}

pub struct World { world: *mut LilvWorld }
pub struct Plugin { plugin: *const LilvPlugin }
pub struct Node { node: *const LilvNode }
pub struct Instance { instance: *mut LilvInstance }
pub struct Port {
    port: *const LilvPort,
    plugin: *const LilvPlugin, // keep a plugin reference for lilv_port methods
}

#[link(name = "lilv-0")]
extern {
    //...
    fn lilv_nodes_free(collection: *const LilvNodes);
    fn lilv_nodes_size(collection: *const LilvNodes) -> u32;
    fn lilv_nodes_begin(collection: *const LilvNodes) -> *mut LilvIter;
    fn lilv_nodes_get(collection: *const LilvNodes, i: *mut LilvIter) -> *const LilvNode;
    fn lilv_nodes_next(collection: *const LilvNodes, i: *mut LilvIter) -> *mut LilvIter;
    fn lilv_nodes_is_end(collection: *const LilvNodes, i: *mut LilvIter) -> bool;
    fn lilv_nodes_get_first(collection: *const LilvNodes) -> *mut LilvNode;

    fn lilv_plugins_size(collection: *const LilvPlugins) -> u32; // -> unsigned
    fn lilv_plugins_begin(collection: *const LilvPlugins) -> *mut LilvIter;
    fn lilv_plugins_get(collection: *const LilvPlugins, i: *mut LilvIter) -> *const LilvPlugin;
    fn lilv_plugins_next(collection: *const LilvPlugins, i: *mut LilvIter) -> *mut LilvIter;
    fn lilv_plugins_is_end(collection: *const LilvPlugins, i: *mut LilvIter) -> bool;
    fn lilv_plugins_get_by_uri(collection: *const LilvPlugins, uri: *const LilvNode) -> *const LilvPlugin;

    fn lilv_world_new() -> *mut LilvWorld;
    fn lilv_world_set_option(world: *mut LilvWorld, uri: *const ::std::os::raw::c_char, value: *const LilvNode);
    fn lilv_world_free(world: *mut LilvWorld);
    fn lilv_world_load_all(world: *mut LilvWorld);
    fn lilv_world_load_bundle(world: *mut LilvWorld, bundle_uri: *const LilvNode);
    fn lilv_world_load_specifications(world: *mut LilvWorld);
    fn lilv_world_load_plugin_classes(world: *mut LilvWorld);
    fn lilv_world_unload_bundle(world: *mut LilvWorld, bundle_uri: *const LilvNode) -> std::os::raw::c_int;
    fn lilv_world_load_resource(world: *mut LilvWorld, resource: *const LilvNode) -> std::os::raw::c_int;
    fn lilv_world_get_all_plugins(world: *mut LilvWorld) -> *const LilvPlugins;

    fn lilv_plugin_verify(plugin: *const LilvPlugin) -> bool;
    fn lilv_plugin_get_uri(plugin: *const LilvPlugin) -> *const LilvNode;
    fn lilv_plugin_get_bundle_uri(plugin: *const LilvPlugin) -> *const LilvNode;
    fn lilv_plugin_get_data_uris (plugin: *const LilvPlugin) -> *const LilvNodes;
    fn lilv_plugin_get_library_uri(plugin: *const LilvPlugin) -> *const LilvNode;
    fn lilv_plugin_get_name(plugin: *const LilvPlugin) -> *mut LilvNode;
    fn lilv_plugin_get_class(plugin: *const LilvPlugin) -> *const LilvPluginClass;
    fn lilv_plugin_get_value(p: *const LilvPlugin, predicate: *const LilvNode) -> *mut LilvNodes;
    fn lilv_plugin_has_feature(p: *const LilvPlugin, feature_uri: *const LilvNode) -> bool;
    fn lilv_plugin_get_supported_features(p: *const LilvPlugin) -> *mut LilvNodes;
    fn lilv_plugin_get_required_features(p: *const LilvPlugin) -> *mut LilvNodes;
    fn lilv_plugin_get_optional_features(p: *const LilvPlugin) -> *mut LilvNodes;
    fn lilv_plugin_has_extension_data(p: *const LilvPlugin, uri: *const LilvNode) -> bool;
    fn lilv_plugin_get_num_ports(p: *const LilvPlugin) -> u32;
    fn lilv_plugin_get_port_ranges_float(p: *const LilvPlugin, min_values: *mut f32, max_values: *mut f32, def_values: *mut f32);
    fn lilv_plugin_get_num_ports_of_class(p: *const LilvPlugin, class_1: *const LilvNode, ...) -> u32;
    fn lilv_plugin_has_latency(p: *const LilvPlugin) -> bool;
    fn lilv_plugin_get_latency_port_index(p: *const LilvPlugin) -> u32;
    fn lilv_plugin_get_port_by_index(plugin: *const LilvPlugin, index: u32) -> *const LilvPort;
    fn lilv_plugin_get_port_by_symbol(plugin: *const LilvPlugin, symbol: *const LilvNode) -> *const LilvPort;
    fn lilv_plugin_get_port_by_designation(plugin: *const LilvPlugin, port_class: *const LilvNode, designation: *const LilvNode) -> *const LilvPort;
    fn lilv_plugin_get_project(plugin: *const LilvPlugin) -> *mut LilvNode;
    fn lilv_plugin_get_author_name(plugin: *const LilvPlugin) -> *mut LilvNode;
    fn lilv_plugin_get_author_email(plugin: *const LilvPlugin) -> *mut LilvNode;
    fn lilv_plugin_get_author_homepage(plugin: *const LilvPlugin) -> *mut LilvNode;

    //...
    fn lilv_node_as_uri(value: *const LilvNode) -> *const ::std::os::raw::c_char;
    fn lilv_node_as_string(value: *const LilvNode) -> *const ::std::os::raw::c_char;
    fn lilv_plugin_instantiate(p: *const LilvPlugin, sample_rate: f64, features: *const &[lv2::LV2Feature]) -> *mut LilvInstance;
    fn lilv_instance_free(instance: *mut LilvInstance);
    fn lilv_instance_activate(instance: *mut LilvInstance);
    fn lilv_instance_run(instance: *mut LilvInstance, sample_rate: u32);
    fn lilv_instance_deactivate(instance: *mut LilvInstance);

    fn lilv_port_get_name(plugin: *const LilvPlugin, port: *const LilvPort) -> *mut LilvNode;
}

impl World {
    pub fn new() -> World {
        unsafe {
            let inner_world = lilv_world_new();
            World { world: inner_world }
        }
    }

    pub fn load_all(&self) {
        unsafe {
            lilv_world_load_all(self.world)
        }
    }

    pub fn get_all_plugins(&self) -> Vec<Plugin> {
        let mut result:Vec<Plugin> = vec![];
        unsafe {
            let plugins = lilv_world_get_all_plugins(self.world);

            let mut iter = lilv_plugins_begin(plugins);
            while !lilv_plugins_is_end(plugins, iter) {
                result.push(Plugin {
                    plugin: lilv_plugins_get(plugins, iter)
                });
                iter = lilv_plugins_next(plugins, iter);
            }
        }
        result
    }
}

impl Drop for World {
    fn drop(&mut self) {
        unsafe {
            lilv_world_free(self.world)
        }
    }
}

impl Plugin {
    pub fn get_name(&self) {

    }

    pub fn get_uri(&self) -> Node {
        unsafe {
            Node { node: lilv_plugin_get_uri(self.plugin) }
        }
    }

    pub fn verify(&self) -> bool {
        unsafe {
            lilv_plugin_verify(self.plugin)
        }
    }

    pub fn get_num_ports(&self) -> u32 {
        unsafe {
            lilv_plugin_get_num_ports(self.plugin)
        }
    }

    pub fn instantiate(&self, sample_rate: f64, features: &[lv2::LV2Feature]) -> Instance {
        unsafe {
            Instance { instance: lilv_plugin_instantiate(self.plugin, sample_rate, &features) }
        }
    }

    pub fn get_port_by_index(&self, index: u32) -> Port {
        unsafe {
            Port {
                port: lilv_plugin_get_port_by_index(self.plugin, index),
                plugin: self.plugin,
            }
        }   
    }
}

impl Instance {
    pub fn get_uri(&self) -> &str {
        unsafe {
            let desc_ptr = (*self.instance).lv2_descriptor;
            let uri_ptr = (*desc_ptr).uri;
            let c_str = ::std::ffi::CStr::from_ptr(uri_ptr);
            let bytes = c_str.to_bytes();
            ::std::str::from_utf8(bytes).unwrap()
        }
    }

    pub fn connect_port(&self, port_index: u32, data_location: *mut libc::c_void) {
        unsafe {
            ((*(*self.instance).lv2_descriptor).connect_port)((*self.instance).lv2_handle, port_index, data_location);
        }
    }

    pub fn activate(&self) {
        unsafe {
            if let Some(activate) = (*(*self.instance).lv2_descriptor).activate {
                activate((*self.instance).lv2_handle);
            }
        }
    }

    pub fn run(&self, sample_count: u32) {
        unsafe {
            ((*(*self.instance).lv2_descriptor).run)((*self.instance).lv2_handle, sample_count);
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            if let Some(deactivate) = (*(*self.instance).lv2_descriptor).deactivate {
                deactivate((*self.instance).lv2_handle);
            }
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { lilv_instance_free(self.instance) }
    }
}

impl Node {
    pub fn as_uri(&self) -> &str {
        unsafe {
            let uri_ptr = lilv_node_as_uri(self.node);
            let c_str = ::std::ffi::CStr::from_ptr(uri_ptr);
            let bytes = c_str.to_bytes();
            ::std::str::from_utf8(bytes).unwrap()
        }
    }

    pub fn as_string(&self) -> &str {
        unsafe {
            let uri_ptr = lilv_node_as_string(self.node);
            let c_str = ::std::ffi::CStr::from_ptr(uri_ptr);
            let bytes = c_str.to_bytes();
            ::std::str::from_utf8(bytes).unwrap()
        }
    }
}

impl Port {
    pub fn get_name(&self) -> Node {
        unsafe {
            Node { node: lilv_port_get_name(self.plugin, self.port) }
        }
    }
}
