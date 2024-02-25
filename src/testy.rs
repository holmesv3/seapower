use leptos_tailwind::types::util::GameState;
fn main() {
    let file = std::path::Path::new("shipmanifest.json");
    let thing: GameState = serde_json::from_reader(std::fs::File::open(file).unwrap()).unwrap();
}
