# Wivlang
---

## Idea
create Language that is a combination of Zig and Rust but with a garbage collector.
I use this project to learn how to implement my own language and usefull librarys for it.

_Wivl_ contains the standart library (std) 
than there will be _zaphyr_, which is a homage to ruby on rails, so its like a server framwork like axum
and _iqni_ which is going to be a way to use async and greenthreads.
Also a a ui library would be cool (name: _acontium_?)
Should be compiled to wasi

How does it look

```zig
pub fn entry() !void {
    let num: i32 = cool_num(5);
    printf("Hello World {num:d}");
}

fn cool_num(n: u32) i32 {
    => (n * 2 + 1) as i32
}

```

---

### Zaphyr & Iqni
```zig
@use zaphyr::*;
@use iqni::*;

pub fn entry () !void{
    zaphyr::bind("127.0.0.1:4242")
        .hook(echo)
        .launch();
}

@get("/")
async fn echo(msg: str) !str{
    => msg
}
```
