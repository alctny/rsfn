use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
/// 自动删除文件名当中的非法字符
/// 1. 会自动跳过隐藏文件和目录
/// 2. 递归目录下的所有文件
struct RsfnArg {
    #[arg(long, default_value_t=String::from("."))]
    /// 需要处理的目录，默认当前目录
    path: String,

    #[arg(long, default_value_t=String::from(""))]
    /// 需要被替换的字符
    pam: String,

    #[arg(long, default_value_t=String::from(""))]
    /// 将 pam 替换为 to，to 默认为 ""，即删除 pam
    to: String,
}

fn main() {
    let arg = RsfnArg::parse();
    rsfn(&(arg.path), &(arg.pam), &(arg.to));
}

fn rsfn(p: &str, pam: &str, to: &str) {
    let dir = fs::read_dir(p).expect("open dir error");
    for file in dir
        .filter_map(Result::ok)
        .filter(|f| f.file_type().is_ok() && !f.file_name().to_string_lossy().starts_with("."))
    {
        match file.file_type().unwrap().is_dir() {
            true => {
                let file_name = &file.file_name().to_string_lossy().to_string();
                let new_file_name = term(&file_name.replace(pam, to));
                let old_path = Path::new(p).join(file_name).to_string_lossy().to_string();
                let new_path = Path::new(p)
                    .join(new_file_name)
                    .to_string_lossy()
                    .to_string();
                fs::rename(old_path, &new_path).unwrap();
                rsfn(&new_path, pam, to);
            }
            false => {
                let file_name = &file.file_name().to_string_lossy().to_string();
                let new_file_name = term(&file_name.replace(pam, to));
                let old_path = Path::new(p).join(file_name).to_string_lossy().to_string();
                let new_path = Path::new(p)
                    .join(new_file_name)
                    .to_string_lossy()
                    .to_string();
                fs::rename(old_path, &new_path).unwrap();
            }
        };
    }
}

fn term(s: &str) -> String {
    let term = vec![
        '　', '（', '）', '，', '；', '！', '：', '”', '“', '、', '。', '【', '】', '(', ')', ' ',
        '[', ']', ':', '「', '」', '\'', '"', '|', '<', '>', '《', '》', ']', '[', ';', '?',
    ];
    let mut res = String::from(s);
    for v in term {
        res = res.replace(v, "");
    }
    res
}
