## Clases

### Notas

* Crear Nuevo proyecto *
```bash
    cargo new <proy_name>
```

* Las variables deben escribirse con snake_case *

* Para permitir a las variables ser mutables *
Se necesita anteponer `mut` a el nombre de la variable

```rust
    let mut var_mut:i16 = 6;
```

* Ingresar datos *
```rust
    std::io::stdin().read_line(&mut age).unwrap();
```

## Tipos de Dato

Numerico con signo 
- i8
- i16
- i32

Numeros sin signo
- u8
- u16
- u32 

Tipo unico ` -> ()`


* Paquetes 
https://crates.io


### Que significa unwrap
 
No tiene tipo de dato nulo, Si un valor puede o no existir se usa un tipo Option<T>, siendo T, el tipo base