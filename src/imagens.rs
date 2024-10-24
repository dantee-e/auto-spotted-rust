use qrcodegen::{QrCode, QrCodeEcc};
use image::{Rgba, RgbaImage, ImageBuffer};
use rusttype::{Font, Scale, point};
use imageproc::drawing::draw_text_mut;
use core::str;
use std::fs;

const TEMPLATE_PATH: &str = "assets/imgs/template.jpeg";
const FONT_PATH: &str = "assets/fonts/Sniglet/Sniglet-Regular.ttf";
const FONT_EMOJI_PATH: &str = "assets/fonts/Noto_Emoji/NotoEmoji-VariableFont_wght.ttf";

const PROPORTION_TEXT_IMG_X: u32 = 25; // em porcentagem
const PROPORTION_TEXT_IMG_Y: u32 = 33; // em porcentagem
const PROPORTION_QR_CODE_IMG: u32 = 35;


enum TipoToken {
    Palavra(String),
    Emoji(char),
    Simbolo(char)
}

fn is_emoji(c: &char) -> bool {
    let emoji_ranges = [
        ('\u{1F600}', '\u{1F64F}'), // Emoticons
        ('\u{1F300}', '\u{1F5FF}'), // Símbolos e pictogramas diversos
        ('\u{1F680}', '\u{1F6FF}'), // Símbolos de transporte e mapas
        ('\u{1F700}', '\u{1F77F}'), // Símbolos alfanuméricos
        ('\u{1F780}', '\u{1F7FF}'), // Símbolos geográficos e outros
        ('\u{1F800}', '\u{1F8FF}'), // Suplemento de pinos de seta e teclado
        ('\u{1F900}', '\u{1F9FF}'), // Suplemento de emojis
        ('\u{1FA00}', '\u{1FA6F}'), // Suplemento de símbolos adicionais
        ('\u{1FA70}', '\u{1FAFF}'), // Suplemento adicional
        ('\u{2702}', '\u{27B0}'),   // Dingbats
        ('\u{24C2}', '\u{1F251}'),  // Enclosed characters
    ];

    emoji_ranges.iter().any(|&(start, end)| start <= *c && *c <= end)
}

fn text_length(font: &Font, scale: &Scale, text: &str) -> i32 {
    let v_metrics = font.v_metrics(*scale);
    let glyphs: Vec<_> = font.layout(text, *scale, point(0.0, v_metrics.ascent)).collect();

    // Calcule a largura total dos glifos
    let width = glyphs.iter().map(|g| g.unpositioned().h_metrics().advance_width).sum::<f32>();

    width as i32// Retorna a largura total do texto
}

fn font_height(font: &Font, scale: &Scale) -> i32 {
    let v_metrics = font.v_metrics(*scale);
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    height as i32
}

fn separate_text(linha: &String) -> impl Iterator<Item = TipoToken> {
    let mut lista: Vec<TipoToken> = Vec::new();
    let mut token = String::new();

    for c in linha.chars(){
        if c.is_alphanumeric() {
            token.push(c);
            continue;
        }
        if !token.is_empty(){
            // println!("{token}");
            lista.push(TipoToken::Palavra(token.clone()));
            token.clear();
        }
        if is_emoji(&c){
            // println!("{}", c);
            lista.push(TipoToken::Emoji(c));
        }
        else{
            // println!("{}", c);
            lista.push(TipoToken::Simbolo(c));
        }
    }

    if !token.is_empty() {
        lista.push(TipoToken::Palavra(token));
    }

    lista.into_iter()
}

fn draw_token(
    token: TipoToken, 
    rgba_img: &mut RgbaImage, 
    color: Rgba<u8>, 
    current_x: &mut i32, 
    current_y: &mut i32, 
    scale: Scale, 
    font: &Font, 
    font_emoji: &Font, 
    mencao: &mut bool, 
    mencoes: &mut Vec<String>,
    linha: &mut String,
){ // Retorna a nova posição X após desenhar o token
    let lower_limit_x: i32 = ((rgba_img.width()*(PROPORTION_TEXT_IMG_X))/100)
        .try_into()
        .expect("Nao converteu u32 para i32");
    let upper_limit_x: i32 = ((rgba_img.width()*(100-PROPORTION_TEXT_IMG_X))/100)
        .try_into()
        .expect("Nao converteu u32 para i32");

    let (texto, fonte) = match token {
        TipoToken::Palavra(palavra) => {
            if *mencao {
                mencoes.push(palavra.clone());
                *mencao = false;
            }
            linha.push_str(&palavra);
            println!("Adicionada {palavra} a linha");
            (palavra, font)
        },
        TipoToken::Emoji(emoji) => {
            linha.push(emoji);
            (emoji.to_string(), font_emoji)
        },
        TipoToken::Simbolo(simbolo) => {
            if simbolo == '@' {
                *mencao = true;
            }
            linha.push(simbolo);
            (simbolo.to_string(), font)
        },
    };

    // Logica de escrever em si
    let try_x = text_length(fonte, &scale, &linha) + lower_limit_x;
    if (try_x + text_length(fonte, &scale, &texto)) > upper_limit_x{
        *current_x = lower_limit_x;
        *current_y += font_height(fonte, &scale);
        linha.clear();
        linha.push_str(&texto);
        *current_x = lower_limit_x;
        println!("Current X = {current_x}");
        println!("Current Y = {current_y}");
        draw_text_mut(rgba_img, color, *current_x, *current_y, scale, fonte, &texto);
    }
    else{
        draw_text_mut(rgba_img, color, *current_x, *current_y, scale, fonte, &texto);
        
        *current_x = text_length(fonte, &scale, &linha) + lower_limit_x;
    }

}


#[allow(dead_code)]
pub fn img_qr_code(spt_num: i32, link: &str) -> String {
    let mut background = image::open(TEMPLATE_PATH).unwrap().into_rgba8();

    // Gera o QR Code usando a crate `qrcodegen`
    let qr = QrCode::encode_text(link, QrCodeEcc::Medium).unwrap();
    
    // Cria um buffer de imagem para o QR code com tamanho 100x100
    let qr_size: u32 = (background.width()*PROPORTION_QR_CODE_IMG)/100;
    let scale: i32 = (qr_size / qr.size() as u32).try_into().expect("size < 0");
    let qr_image = ImageBuffer::from_fn(qr_size, qr_size, |x, y| {
        let is_dark = qr.get_module(x as i32 / scale, y as i32 / scale);
        if is_dark {
            Rgba([0u8, 0u8, 0u8, 255u8]) // Pixel preto
        } else {
            Rgba([255u8, 255u8, 255u8, 255u8]) // Pixel branco
        }
    });

    // Carrega a imagem de fundo
    

    // Define a posição onde o QR Code será sobreposto
    let x_position: i64 = ((background.width()-qr_size)/2).try_into().expect("Erro convertendo slaoq"); // Posição X
    let y_position = ((background.height()-qr_size)/2).try_into().expect("Erro convertendo slaoq");

    // Sobrepõe o QR Code na imagem de fundo
    image::imageops::overlay(&mut background, &qr_image, x_position, y_position);

    // Salva a nova imagem
    let output_file = format!("assets/imgs/outputs/qr_code_output_{}.jpg", spt_num);
    background.save(&output_file).unwrap();

    // Retorna o caminho do arquivo salvo
    output_file
}

#[allow(dead_code)]
pub fn write_text(spt_num: i32, text: &String, font_size: f32) -> String {
    // Carrega a imagem do template
    let img = image::open(TEMPLATE_PATH).expect("Failed to open template");
    
    // Converte a imagem para RgbaImage (ImageBuffer<Rgba<u8>, Vec<u8>>)
    let mut rgba_img = img.to_rgba8();

    // Carrega a fonte
    let font_data = fs::read(FONT_PATH).expect("Failed to read font file");
    let font = Font::try_from_vec(font_data).expect("Error constructing Font");

    let font_emoji_data = fs::read(FONT_EMOJI_PATH).expect("Failed to read font file");
    let font_emoji = Font::try_from_vec(font_emoji_data).expect("Error constructing Font");

    // Tamanho e cor do textotext_length(fonte, &scale, &texto) + *current_x
    let scale = Scale { x: font_size, y: font_size }; // Tamanho do texto
    let color = Rgba([0, 0, 0, 0]); // Cor preta

    // Desenha o texto na imagem
    let texto = separate_text(&text);
    let mut current_x: i32 = ((rgba_img.width()*PROPORTION_TEXT_IMG_X)/100).try_into().expect("Nao converteu u32 para i32");
    let mut current_y: i32 = ((rgba_img.height()*PROPORTION_TEXT_IMG_Y)/100).try_into().expect("Nao converteu u32 para i32");

    

    let mut linha = String::new();
    let mut mencao = false;

    let mut mencoes: Vec<String> = Vec::new();

    for token in texto {
        draw_token(token, &mut rgba_img, color, &mut current_x, &mut current_y, scale, &font, &font_emoji, &mut mencao, &mut mencoes, &mut linha);
    }


    let output_path = format!("assets/imgs/outputs/spotted-{spt_num}.png");
    rgba_img.save(&output_path).expect("Failed to save image with text");

    output_path
}



