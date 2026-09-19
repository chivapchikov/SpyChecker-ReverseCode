
use std::{fs, u16};
mod interface;


fn main() -> std::io::Result<()> {


    interface::app::run_gui().expect("Error GUI");

    //  прочтения файла и перевод в элементы вектора 
    let bin: Vec<u8> = fs::read("Main.class")?;
    // получение вектора с байтами

    // проверка на слишком маленький файл 
    if bin.len() < 4 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "File small, size"));
    }
  
  
    
    // проверка магических байтов если они не валидны завершение  
    // протестировать !!!
    let magic: u32 = u32::from_be_bytes([bin[0], bin[1], bin[2], bin[3]]);
     if magic != 0xCAFEBABE {
          return Err(std::io::Error::new(std::io::ErrorKind::Other, "Magic Number, failed"));
    }
     // проверка на версию, если файл меньше 
    if bin.len() < 8 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "File small, i can not read version"));
    }
    
    // проверка на pool const - это место где хранится информация небоходимая для работы файла.
    if bin.len() < 10 {
        return  Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "File small, i can not read pool const"));
    }
    
                 let raw_major = u16::from_be_bytes([bin[6], bin[7]]);
                  let java_version = if raw_major >= 45 {
                      format!("Java {}", raw_major - 44)
                        } else {
                           "Неизвестная версия".to_string()
                        };

        
                let count_const_pool = u16::from_be_bytes([bin[8], bin[9]]) - 1; // важные для работы файла данные  пул констанст
            

                let mut cursor = 10; // cursor хранит текущий индекс байта 
                // после 10 байта начинается таблица констант где элементы имеют разный размер


    // class_index — где искать метод (в каком классе) [11, 12 байт]
    // name_and_type_index — какой метод искать (имя + типы аргументов) [13, 14 байт]
    // курсор это наш индекс мы должны выполнять проверку если индекс курсора привышает индекс который есть в векторе = ошибка

                let mut i = 1;
      
                while i <= count_const_pool {

                    if cursor >= bin.len() {
                         return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, damaged"));
                    }

                    let tag = bin[cursor];

                    match tag  {
                        10 => { // тэг 10 хранит 5 байт class index, name_and_type_index.
                        if bin.len() - cursor < 5  {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let class_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                        let name_and_type_index = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]); 

                       println!("Methodref: class={}, name_and_type={}", class_index, name_and_type_index); 
                       i += 1;
                       cursor += 5;
                    },

                     7 => {
                          if bin.len() - cursor < 3 {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let name_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                        i += 1;
                        cursor += 3;
                     },

                     1 => {
                         if bin.len() - cursor < 3 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                           let length = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]) as usize;
                            if bin.len() < cursor + 3 + length {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        } 
                      let line = &bin[cursor + 3..cursor + 3 + length];
                                              
                            let utf8_str = String::from_utf8_lossy(line);
                            println!("[#{}] Utf8: {}", i, utf8_str);



                         cursor += 3 + length;
                          i += 1;
                     }
                     12 => {
                         if bin.len() - cursor < 5 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                       let name_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                        let descrptor_index = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]);
                        cursor += 5;
                        i+=1;

                     }
                     9 => {
                          if bin.len() - cursor < 5 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let class_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                        let name_and_type_index = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]);
                        cursor += 5;
                          i += 1;
                     }
                     8 => { // String
                         if bin.len() - cursor < 3 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                         let cstring_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                         i += 1;
                         cursor += 3;
                     }
                     
                     3 => { // intger
                          if bin.len() - cursor < 5 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        
                        let integer = i32::from_be_bytes([
                           bin[cursor + 1],
                             bin[cursor + 2],
                                 bin[cursor + 3],
                                        bin[cursor + 4],
                                ]); // берем 4 байта т.к int это 32 бита / 8 = 4 байта; 

                        cursor += 5;
                        i += 1;
                     }
                     4 => { // float
                        if bin.len() - cursor < 5 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }

                        let float = u32::from_be_bytes([
                           bin[cursor + 1],
                             bin[cursor + 2],
                                 bin[cursor + 3],
                                        bin[cursor + 4],
                                        ]); // так же получаем 4 байта
                        let finfloat = f32::from_bits(float); // интерпретируем 4 байта как число с плавающей точкой (f32)
                            cursor += 5;
                            i += 1;
                     }

                     5 => { // long
                      if bin.len() - cursor < 9 {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                      }

                      let long = i64::from_be_bytes([ 
                        bin[cursor + 1],
                         bin[cursor + 2 ],
                          bin[cursor + 3 ],
                           bin[cursor + 4 ],
                            bin[cursor + 5 ],
                             bin[cursor + 6 ],
                              bin[cursor + 7 ],
                               bin[cursor + 8 ]
                         ]);
                      i += 2;
                      cursor += 9;
                     } 

              6 => { // double
                 if bin.len() - cursor < 9 {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                  }

                  let double = u64::from_be_bytes([
                    bin[cursor + 1],
                     bin[cursor + 2],
                      bin[cursor + 3],
                       bin[cursor + 4],
                        bin[cursor + 5],
                         bin[cursor + 6],
                          bin[cursor + 7],
                           bin[cursor + 8],
                    ]);
                    let findouble = f64::from_bits(double);
                  i+=2;
                  cursor += 9;
              }

              11 => { // Интерфейсы
                if bin.len() - cursor < 5 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }
                let class_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                let name_and_type_index = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]);
                cursor += 5;
                 i += 1; 
              }

              15 => { // MethodHandle

                if bin.len() - cursor < 4 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }

                  let reference_kind = u8::from_be_bytes([bin[cursor + 1]]);
                    let reference_index = u16::from_be_bytes([bin[cursor + 2], bin[cursor + 3]]);

                cursor += 4;
                i += 1;

              }

              16 => { // MethodType
                if bin.len() - cursor < 3 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }
                
                    let descriptor_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                
                cursor += 3;
                i+=1;

              }

              17 => { // Dynamic

                  if bin.len() - cursor < 5 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }

                let bootstrap_attr_idx = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                let name_and_type_idx = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]);


                 cursor += 5;
                i+=1;
                
              }

              18 => { // InvokeDynamic

                if bin.len() - cursor < 5 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }


                 let bootstrap_attr_idx = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                let name_and_type_idx = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]);
                 cursor += 5;
                i+=1;


              }

              19 => { // Module

                  if bin.len() - cursor < 3 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }

                     let name_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);

                 cursor += 3;
                i+=1;
              }

              20 => { // Package

                    if bin.len() - cursor < 3 {
                  return Err(std::io::Error::new(std::io::ErrorKind::Other, "File erorr, Cursor out of bounds"));
                }

                     let name_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);

                 cursor += 3;
                i+=1;


              }

                     


                _ => {
                     println!("Неизвестный тег {} на смещении {}", tag, cursor);
                     break;
                }

             }


       }





  Ok(())
}

 
            
