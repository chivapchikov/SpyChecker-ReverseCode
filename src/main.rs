use std::{fs, u16};

fn main() -> std::io::Result<()> {

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
    
                // проверка java
         let major_version = u16::from_be_bytes([bin[6], bin[7]]);

                let java = match major_version {
                    52 => "Java 8",
                    55 => "Java 11",
                    61 => "Java 17",
                    65 => "Java 21",
                    _ => "Неизвестная версия",
                };

        
                let count_const_pool = u16::from_be_bytes([bin[8], bin[9]]) - 1; // важные для работы файла данные  пул констанст
            
                let mut cursor = 10; // cursor хранит текущий индекс байта 
                // после 10 байта начинается таблица констант где элементы имеют разный размер


    // class_index — где искать метод (в каком классе) [11, 12 байт]
    // name_and_type_index — какой метод искать (имя + типы аргументов) [13, 14 байт]
    // курсор это наш индекс мы должны выполнять проверку если индекс курсора привышает индекс который есть в векторе = ошибка




            for i in 0..count_const_pool { // тут у нас идет цикл от 0 до полученого количества байт 

             if bin.len() <= cursor {

                // Условно у нас в bin.len 10 элементов (от 0 до 9 по индексу ), а cursor = 10 это и есть наш указатель на индекс, в bin.len нет 10 элемента по этому будет ошибка!
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
             }


// в 10 индексе хранятся индексы класса и индексы метода(5 байт), в 7 входят индексы  классов интерфейсов, и масивов строк и чисел, а в 1 строки



                let tag = bin[cursor]; // тут tag это как наш экран мы передаем идекс в вектор и нам отдает его значение байтов 
                match tag {
                    10 => { // тэг 10 хранит 5 байт class index, name_and_type_index.
                        if bin.len() < cursor + 5 {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let class_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);

                        let name_and_type_index = u16::from_be_bytes([bin[cursor + 3], bin[cursor + 4]]); 

                       println!("[#{}] Methodref: class={}, name_and_type={}", i + 1, class_index, name_and_type_index); // i + 1 нужно чтоб индекс был 1 потому что 0 зарезервирован jvm
                       cursor += 5;

                    },
                    7 => { // 7 это константа класса имеет размер 3 байта 
                        if bin.len() < cursor + 3 {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let name_index = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]);
                        cursor += 3;
                        println!("[#{}] Class: name_index={}", i + 1, name_index);
                    }
                    1 => {
                        if bin.len() < cursor + 3 {
                          return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let length = u16::from_be_bytes([bin[cursor + 1], bin[cursor + 2]]) as usize;
                        println!("{}", length);

                        if bin.len() < cursor + 3 + length {
                            return Err(std::io::Error::new(std::io::ErrorKind::Other, "File error, Cursor out of bounds"));
                        }
                        let line = &bin[cursor + 3..cursor + 3 + length]; // тут мы делаем срез записываем байты по индексу от x до y 
                        // берем ссылку на вектор с байтами и смотрим байты по индексу от x + 3 до x + 3 + y и записываем их в line
                        //  после чего преобразуем в utf8
                        let utf8_str = std::str::from_utf8(line);
                         cursor += 3 + length;
 




                         // продолжить тут !!!

                    }


                    _ => { 
                        println!("Неизвестный тег {} на позиции {}", tag, cursor);
                        break;
                     }


                }






            }







                Ok(())
            
    }
