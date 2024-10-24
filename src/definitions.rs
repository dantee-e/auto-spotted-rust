const SPOTTED_NUMBER_FILE_PATH: &str = "assets/numero_spotted.txt";


pub struct SpottedStruct {
    pub id: String,
    pub spt_num: i32,
    pub mensagem: String,
    pub identificacao: String,
    pub link: String,
}
impl SpottedStruct {
    pub fn new() -> SpottedStruct {
        SpottedStruct {
            id: String::new(),
            spt_num: 0,
            mensagem: String::new(),
            identificacao: String::new(),
            link: String::new()
        }
    }
}

pub fn get_spotted_nmr() -> i32 {
    let file_contents = std::fs::read_to_string(SPOTTED_NUMBER_FILE_PATH)
        .expect("Failed to read the spotted number");
    file_contents.parse::<i32>().unwrap_or_default()
}

pub fn update_spotted_nmr() -> bool {
    let file_contents = std::fs::read_to_string(SPOTTED_NUMBER_FILE_PATH)
        .expect("Failed to read the spotted number");
    let mut current = file_contents.parse::<i32>().unwrap_or_default();
    current += 1;
    match std::fs::write(SPOTTED_NUMBER_FILE_PATH, current.to_string()) {
        Ok(_) => true,
        Err(e) => {
            println!("Erro: {e}");
            false
        }
    }
}