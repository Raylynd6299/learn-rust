use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::fs;

const FILENAME: &str = "history.csv";

#[derive(Debug)]
struct HistoryStep {
    data_type: String,
    text: String,
    tag: String,
    life: i32,
    options: Vec<HistoryStep>,
}

trait HistoryStepInterface {
    fn new(row: StringRecord) -> HistoryStep;
}

impl HistoryStepInterface for HistoryStep {
    fn new(row: StringRecord) -> HistoryStep {
        return HistoryStep {
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            life: row.get(3).unwrap().trim().parse().unwrap_or(0),
            options: vec![],
        };
    }
}

fn main() {
    let mut user_life = 100;
    let mut actual_tag: &str = "INICIO";

    let mut last_record: String = "".to_string();

    let mut steps_history: HashMap<String, HistoryStep> = HashMap::new();

    let content = fs::read_to_string(FILENAME).unwrap();

    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        let result = result.unwrap();
        let step = HistoryStep::new(result);

        if step.data_type == "SITUACION" {
            let record_tag = step.tag.clone();

            steps_history.insert(record_tag.clone(), step);

            last_record = record_tag;
        } else if step.data_type == "OPCION" {
            if let Some(data) = steps_history.get_mut(&last_record) {
                (*data).options.push(step);
            }
        }
    }

    loop {
        println!("Vida: {user_life}pt");

        if let Some(data) = steps_history.get(actual_tag) {
            println!("{}", data.text);

            for (index, opt) in data.options.iter().enumerate() {
                println!("[{}] {}", index, opt.text);
            }

            let mut select_opt = String::new();

            std::io::stdin().read_line(&mut select_opt).unwrap();
            let selection = select_opt.trim().parse().unwrap_or(9999);

            if let Some(opcion_eleg) = &data.options.get(selection){
                actual_tag = &opcion_eleg.tag
            }  else {
                println!("Opcion invalida")
            }

            user_life += data.life;
            println!("");

        }else{
            break;
        }

        if user_life <= 0 {
            println!("YOU LOSE!!!");
            break;
        }
    }
}
