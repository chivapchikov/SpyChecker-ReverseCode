use std::fmt::Error;
use std::{fs, u16};
use std::io::ErrorKind;

fn main() -> std::io::Result<()> {
    let bin: Vec<u8> = fs::read("Main.class")?;
    // получение вектора с байтами
    if bin.len() > 0 {
        if bin.len() < 4 {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Файл слишком мал",
            ))?
        } else {
            let magic: u32 = u32::from_be_bytes([bin[0], bin[1], bin[2], bin[3]]);
            // захват 4 байтов (первых)
            if magic != 0xCAFEBABE {
                // проверка на валидный MagicNumber
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Неверный Magic Number",
                ))?
            } else {
                if bin.len() < 8 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Файл слишком мал для считывания версий",
                        // проверка файла на размер
                    ))
                }
                // проверка java ;
                let minor_version = u16::from_be_bytes([bin[4], bin[5]]);
                let major_version = u16::from_be_bytes([bin[6], bin[7]]);

                let java = match major_version {
                    52 => "Java 8",
                    55 => "Java 11",
                    61 => "Java 17",
                    65 => "Java 21",
                    _ => "Неизвестная версия",
                };
                println!("{}", java);


                Ok(())
            }
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Файл пуст",
        ))?
    }
}
