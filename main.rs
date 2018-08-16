use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;


fn visit_dirs(dir: &Path) -> io::Result<()> 
{
    let mut i = 0;
    if dir.is_dir() 
    {
        for entry in fs::read_dir(dir)? 
        {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() 
            {
                //visit_dirs(&path); 폴더안에 하위폴더에 파일들까지 출력
            } else 
            {
                i+=1;
                p(&path);
            }
        }
        println!("\n파일갯수 : {}", i);
    }
    Ok(())
}

fn p(path: &Path)
{
    match path.file_name()
    {
        Some(x) =>
        {
            println!("{:?}", x);
        }
        None =>
        {
            print!("err");
        }
    }
}

fn main()
{
    let mut buf = String::new();
    let mut check = String::new();
    
    println!("파일의 목록을 보고싶은 폴더경로를 입력해주세요.");
    io::stdin().read_line(&mut buf);
    
    let result = buf.trim();
    let path = Path::new(&result);
    
    println!("\n{:?} 폴더의 파일 목록\n",path);
    visit_dirs(&path);
}

