use std::collections::HashMap;
use crate::{common::*, utils::lead_object::*};

pub struct PropertyList{
    children: Vec<LeadObject>,
    strings: HashMap<String, String>,
    floats: HashMap<String, f32>,
    ints: HashMap<String, i32>,
    bools: HashMap<String, bool>,
    point_2s: HashMap<String, Point2f>,
    point_3s: HashMap<String, Point3f>,
    vector_2s: HashMap<String, Vector2f>,
    vector_3s: HashMap<String, Vector3f>,

    transform_matrix: Matrix4x4,
}

impl PropertyList{
    pub fn new() -> Self {
        PropertyList{
            children: Vec::new(),

            strings: HashMap::new(),
            floats: HashMap::new(),
            ints: HashMap::new(),
            bools: HashMap::new(),
            point_2s: HashMap::new(),
            point_3s: HashMap::new(),
            vector_2s: HashMap::new(),
            vector_3s: HashMap::new(),
            transform_matrix: Matrix4x4::identity()
        }
    }

    pub fn is_property_type(name: &str) -> bool {
        match name {
            "string" => true,
            "float" => true,
            "int" => true,
            "bool" => true,
            "point2" | "point3" => true,
            "vector2" | "vector3" => true,

            // For transform
            "scale" => true,
            "translate" => true,
            "rotate" => true,
            _ => false
        }
    }

    pub fn is_property_valid(name: &str, attrs: &HashMap<String, String>) -> bool {
        match name {
            "string" | "float" | "int" | "bool" => attrs.contains_key("name") && attrs.contains_key("value"),
            "point2" | "point3" | "vector2" | "vector3" => attrs.contains_key("name") && attrs.contains_key("value"),
            "scale" | "translate" => attrs.contains_key("value"),
            "rotate" => attrs.contains_key("axis") && attrs.contains_key("angle"),
            _ => false
        }
    }

    pub fn add_property(&mut self, p_type: &str, attrs: &HashMap<String, String>) {
        // if it is a transform, early return it
        if p_type == "rotate" {
            let angle = attrs.get("angle").expect("Angle not provided").to_owned();
            let axis = attrs.get("axis").expect("Axis not provied").to_owned();
            self.add_rotation(angle, axis);
            return;
        } else if p_type == "scale" {
            let value = attrs.get("value").expect("No value provided to prop").to_owned();
            self.add_scale(value);
            return;
        } else if p_type == "translate" {
            let value = attrs.get("value").expect("No value provided to prop").to_owned();
            self.add_translation(value);
            return;
        }

        let key = attrs.get("name").expect("No name provided to prop").to_owned();
        let value = attrs.get("value").expect("No value provided to prop").to_owned();
        match p_type {
            "string" => self.set_string(key, value),
            "int" => {
                if let Ok(int_value) = value.parse::<i32>() {
                    self.set_int(key, int_value);
                }
            }
            "float" => {
                if let Ok(float_value) = value.parse::<f32>() {
                    self.set_float(key, float_value);
                }
            }
            "bool" => {
                match value.as_str() {
                    "true" => self.set_bool(key, true),
                    "false" => self.set_bool(key, false),
                    _ => panic!("Unable to parse {value} to a bool!")
                }
            }
            "point2" => {
                self.set_point2(key, value);
            }
            "point3" => {
                self.set_point3(key, value);
            }
            "vector2" => {
                self.set_vector2(key, value);
            }
            "vector3" => {
                self.set_vector3(key, value);
            }
            _ => {}
        }
    }

    pub fn add_child(&mut self, child: LeadObject) {
        self.children.push(child);
    }

    pub fn set_string(&mut self, k: String, v: String) {
        self.strings.insert(k, v);
    }

    pub fn set_float(&mut self, k: String, v: f32) {
        self.floats.insert(k, v);
    }
    
    pub fn set_int(&mut self, k: String, v: i32) {
        self.ints.insert(k, v);
    }

    pub fn set_bool(&mut self, k: String, v: bool) {
        self.bools.insert(k, v);
    }
    
    pub fn set_point2(&mut self, k: String, v: String) {
        self.point_2s.insert(k, Point2f::init_string(v));
    }
    
    pub fn set_point3(&mut self, k: String, v: String) {
        self.point_3s.insert(k, Point3f::init_string(v));
    }
    
    pub fn set_vector2(&mut self, k: String, v: String) {
        self.vector_2s.insert(k, Vector2f::init_string(v));
    }
    
    pub fn set_vector3(&mut self, k: String, v: String) {
        self.vector_3s.insert(k, Vector3f::init_string(v));
    }

    pub fn add_scale(&mut self, v: String) {
        let scaling_vector = Vector3f::init_string(v);
        let scaling_transform = Transform::scale(&scaling_vector);

        self.transform_matrix = Matrix4x4::mul(&scaling_transform.get_matrix(), &self.transform_matrix);
    }

    pub fn add_translation(&mut self, v: String) {
        let translating_vector = Vector3f::init_string(v);
        let translating_transform = Transform::translate(&translating_vector);

        self.transform_matrix = Matrix4x4::mul(&translating_transform.get_matrix(), &self.transform_matrix);
    }

    pub fn add_rotation(&mut self, angle: String, axis: String) {
        let rotation_axis = Vector3f::init_string(axis);
        let rotation_angle = angle.parse::<f32>().unwrap();
        let rotating_transform = Transform::rotate(rotation_angle, &rotation_axis);

        self.transform_matrix = Matrix4x4::mul(&rotating_transform.get_matrix(), &self.transform_matrix);
    }
    
    pub fn get_string(&self, k: &str, default: &str) -> String {
        self.strings.get(k).cloned().unwrap_or(default.to_string())
    }

    pub fn get_float(&self, k: &str, default: f32) -> f32 {
        self.floats.get(k).cloned().unwrap_or(default)
    }

    pub fn get_int(&self, k: &str, default: i32) -> i32 {
        self.ints.get(k).cloned().unwrap_or(default)
    }
    
    pub fn get_bool(&self, k: &str, default: bool) -> bool {
        self.bools.get(k).cloned().unwrap_or(default)
    }

    pub fn get_point2(&self, k: &str, default: Point2f) -> Point2f {
        self.point_2s.get(k).cloned().unwrap_or(default)
    }

    pub fn get_point3(&self, k: &str, default: Point3f) -> Point3f {
        self.point_3s.get(k).cloned().unwrap_or(default)
    }

    pub fn get_vector2(&self, k: &str, default: Vector2f) -> Vector2f {
        self.vector_2s.get(k).cloned().unwrap_or(default)
    }

    pub fn get_vector3(&self, k: &str, default: Vector3f) -> Vector3f {
        self.vector_3s.get(k).cloned().unwrap_or(default)
    }

    pub fn get_transform(&self) -> Transform {
        Transform::init_mat(&self.transform_matrix)
    }
}