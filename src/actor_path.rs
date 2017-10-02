
pub struct ActorPath {
    pub path_string: String,
}

impl ActorPath {
    pub fn from_string(path: String) -> ActorPath {
        ActorPath { path_string: path }
    }
}
