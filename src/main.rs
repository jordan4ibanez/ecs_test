use glam::Vec3;

#[derive(Debug)]
struct Name(&'static str);
impl Name {
    pub fn value(&self) -> String {
        self.0.to_string()
    }
}
#[derive(Debug)]
struct Health(i32);
#[derive(Debug)]
struct Pos(Vec3);
#[derive(Debug)]
struct Mesh(i32);

#[derive(Debug)]
struct World {
    name_components: Vec<Option<Name>>,
    health_components: Vec<Option<Health>>,
    pos_components: Vec<Option<Pos>>,
    mesh_component: Vec<Option<Mesh>>
}

impl World {
    pub fn new() -> Self {
        Self {
            name_components: Vec::new(),
            health_components: Vec::new(),
            pos_components: Vec::new(),
            mesh_component: Vec::new()
        }
    }

    pub fn new_entity(&mut self,
        name: Option<Name>,
        health: Option<Health>,
        pos: Option<Pos>,
        mesh: Option<Mesh>
    ){
        self.name_components.push(name);
        self.health_components.push(health);
        self.pos_components.push(pos);
        self.mesh_component.push(mesh);
    }

    pub fn remove_entity(&mut self, index: usize) {

        match self.name_components.get(index).clone().unwrap() {
            Some(name) => println!("{} WAS KILLED!", name.value().clone().to_uppercase()),
            None => println!("Entity {} was killed", index)
        };

        self.name_components.remove(index);
        self.health_components.remove(index);
        self.pos_components.remove(index);
        self.mesh_component.remove(index);
    }
}


fn main() {
    let mut test_world  = World::new();

    test_world.new_entity(Some(Name("fred")), Some(Health(1)), Some(Pos(Vec3::new(1.0, 3.0, 4.0))), None);
    test_world.new_entity(Some(Name("jack")), Some(Health(10)), Some(Pos(Vec3::new(3.0, 4.0, 5.0))), None);
    test_world.new_entity(Some(Name("bob")), Some(Health(166)), Some(Pos(Vec3::new(1.1, 0.0, -4.0))), Some(Mesh(33)));

    test_world.remove_entity(1);

    println!("{:#?}", test_world);
}
