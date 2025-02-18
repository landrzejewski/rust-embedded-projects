pub fn run() {
    let json = r#"
    {"city":{"id":756135,"name":"Warsaw","coord":{"lon":21.0118,"lat":52.2298},"country":"PL","population":1000000,"timezone":3600},
     "cod":"200",
     "message":0.0400762,
     "cnt":7,
     "list":[
       {"dt":1739872800,"sunrise":1739857541,"sunset":1739894062,
         "temp":{"day":-2.49,"min":-5.3,"max":-1.98,"night":-3.38,"eve":-1.98,"morn":-4.68},
         "feels_like":{"day":-7.67,"night":-7.68,"eve":-5.63,"morn":-9.26},
         "pressure":1027,"humidity":68,
         "weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":"04d"}],
         "speed":4.45,"deg":267,"gust":8.36,"clouds":81,"pop":0
       },
       {"dt":1739959200,"sunrise":1739943819,"sunset":1739980575,
         "temp":{"day":-0.69,"min":-4.25,"max":1.47,"night":-1.56,"eve":-0.43,"morn":-4.24},
         "feels_like":{"day":-4.34,"night":-1.56,"eve":-2.81,"morn":-8.84},
         "pressure":1032,"humidity":75,
         "weather":[{"id":803,"main":"Clouds","description":"broken clouds","icon":"04d"}],
         "speed":3.53,"deg":264,"gust":9.26,"clouds":83,"pop":0
       },
       {"dt":1740045600,"sunrise":1740030095,"sunset":1740067087,
         "temp":{"day":-0.01,"min":-3.27,"max":1.06,"night":-2.23,"eve":-0.81,"morn":-2.88},
         "feels_like":{"day":-1.92,"night":-6.01,"eve":-3.59,"morn":-4.82},
         "pressure":1035,"humidity":55,
         "weather":[{"id":800,"main":"Clear","description":"sky is clear","icon":"01d"}],
         "speed":2.83,"deg":142,"gust":7.12,"clouds":7,"pop":0
       },
       {"dt":1740132000,"sunrise":1740116370,"sunset":1740153598,
         "temp":{"day":-0.14,"min":-3.84,"max":1.02,"night":-1.26,"eve":0.69,"morn":-3.51},
         "feels_like":{"day":-5.41,"night":-5.76,"eve":-3.65,"morn":-8.46},
         "pressure":1035,"humidity":40,
         "weather":[{"id":801,"main":"Clouds","description":"few clouds","icon":"02d"}],
         "speed":5.57,"deg":157,"gust":11.37,"clouds":18,"pop":0
       },
       {"dt":1740218400,"sunrise":1740202644,"sunset":1740240110,
         "temp":{"day":0.21,"min":-1.88,"max":2.9,"night":-0.76,"eve":2.09,"morn":-1.8},
         "feels_like":{"day":-4,"night":-5.05,"eve":-1.96,"morn":-6.23},
         "pressure":1030,"humidity":46,
         "weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04d"}],
         "speed":4.44,"deg":141,"gust":10.61,"clouds":100,"pop":0
       },
       {"dt":1740304800,"sunrise":1740288917,"sunset":1740326621,
         "temp":{"day":0.28,"min":-2.21,"max":2.65,"night":-0.96,"eve":2.07,"morn":-1.84},
         "feels_like":{"day":-4.74,"night":-5.45,"eve":-1.98,"morn":-6.56},
         "pressure":1035,"humidity":31,
         "weather":[{"id":800,"main":"Clear","description":"sky is clear","icon":"01d"}],
         "speed":5.31,"deg":148,"gust":11.24,"clouds":5,"pop":0
       },
       {"dt":1740391200,"sunrise":1740375189,"sunset":1740413131,
         "temp":{"day":1.87,"min":-1.98,"max":5.5,"night":2.7,"eve":4.98,"morn":-1.9},
         "feels_like":{"day":-2.03,"night":-0.84,"eve":2.25,"morn":-6.25},
         "pressure":1032,"humidity":39,
         "weather":[{"id":800,"main":"Clear","description":"sky is clear","icon":"01d"}],
         "speed":4.06,"deg":166,"gust":11.61,"clouds":0,"pop":0
       }
     ]
    }
    "#;

    let temperatures = extract_day_temperatures(json);
    println!("{:?}", temperatures);
}

fn extract_day_temperatures(json: &str) -> Vec<f64> {
    let mut temps = Vec::new();

    // Find the start of the "list" array.
    if let Some(list_index) = json.find("\"list\":") {
        let list_part = &json[list_index..];

        let mut search_from = 0;
        while let Some(temp_index) = list_part[search_from..].find("\"temp\":{") {
            let temp_start = search_from + temp_index + "\"temp\":".len();

            // Find the end of the temp object (assumes no nested braces).
            if let Some(temp_end_offset) = list_part[temp_start..].find('}') {
                let temp_end = temp_start + temp_end_offset;
                let temp_object = &list_part[temp_start..=temp_end];

                // Look for the "day" value within the temp object.
                if let Some(day_index) = temp_object.find("\"day\":") {
                    let day_start = day_index + "\"day\":".len();
                    // Extract until the next comma or closing brace.
                    let rest = &temp_object[day_start..];
                    let day_end = rest.find(|c: char| c == ',' || c == '}')
                        .unwrap_or(rest.len());
                    let day_str = rest[..day_end].trim();

                    // Convert the extracted string to f64.
                    if let Ok(day_temp) = day_str.parse::<f64>() {
                        temps.push(day_temp);
                    }
                }
                search_from = temp_end + 1;
            } else {
                break;
            }
        }
    }
    temps
}