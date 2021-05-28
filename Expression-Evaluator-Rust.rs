#[derive(Debug, PartialEq)]
#[derive(Clone)]
enum Token {
    Num(i64),
    Operador(char),
    Simbolo(char)
}

#[derive(Clone)]
struct Arvore{
    valor: Token,
    direito: Option<Box<Arvore>>,
    esquerda: Option<Box<Arvore>>,
}

#[allow(unused)]
fn cria_arvore(polonesa : Vec<&Token>) -> Vec<Arvore> {
    let mut pilha: Vec<Arvore> = Vec::new();
    for aux in polonesa{
        match aux {
            &Token::Num(x) =>{
                pilha.push(Arvore{direito: None, esquerda:None, valor: aux.clone()});
            }
            &Token::Operador(x) =>{
                let a = pilha.pop();
                let b = pilha.pop();
                pilha.push(Arvore{direito: Some(Box::new(a.unwrap())), esquerda: Some(Box::new(b.unwrap())), valor: aux.clone()});
            }
            &Token::Simbolo(x) =>{
                println!("Erro");
            }
        }
    }
    pilha
}

#[allow(unused)]
fn precedencia (first: &Token, second: &Token) -> bool{
    let mut first_valor = 0;
    let mut second_valor = 0;
    match first {
        &Token::Num(x) =>{
            first_valor = 100;
        }
        _=>{
            if first == &Token::Operador('+'){
                first_valor = 0;
            }
            else if first == &Token::Operador('-'){
                first_valor = 0;
            }
            else if first == &Token::Operador('*'){
                first_valor = 1;
            }
            else if first == &Token::Operador('/'){
                first_valor = 1;
            }
                }
    }
    match second {
        &Token::Num(x) =>{
            first_valor = -100;
        }
        _=>{
            if second == &Token::Operador('+'){
                second_valor = 0;
            }
            else if second == &Token::Operador('-'){
                second_valor = 0;
            }
            else if second == &Token::Operador('*'){
                second_valor = 1;
            }
            else if second == &Token::Operador('/'){
                second_valor = 1;
            }
        }
    }
    let result: bool = first_valor >= second_valor;
    result
}

#[allow(unused)]
fn lex(entrada: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = entrada.chars().peekable();
    while let Some(&c) = iter.peek() {
        match c {
            c if c.is_numeric() => {
                let mut digits = String::new();
                while let Some(&c) = iter.peek() {
                    if c.is_numeric() {
                        digits.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Num(digits.parse().unwrap()));
            }
            '+' | '*' | '/'  => {
                iter.next();
                tokens.push(Token::Operador(c));
            }
            '-' => {
                iter.next();
                let d = iter.peek().unwrap();
                if d.is_numeric(){
                    let mut digits = String::new();
                    while let Some(&c) = iter.peek(){
                        if c.is_numeric() {
                            digits.push(c);
                            iter.next();
                        }
                        else if c == '-'{
                            iter.next();
                            tokens.push(Token::Operador(c));
                        }
                        else {
                            break;
                        }
                    }
                    let aux = digits.parse::<i64>().unwrap();
                    let _aux = aux * -1;
                    tokens.push(Token::Num(_aux))
            }
            else{
                iter.next();
                tokens.push(Token::Operador(c));
            }
            }
            
            '(' => {
                iter.next();
                tokens.push(Token::Simbolo(c));
                
            }
            ')' => {
                iter.next();
                tokens.push(Token::Simbolo(c));
            }
            _ => {
                iter.next();
            }
        }
    }
    tokens
}

#[allow(unused)]
fn shunting_yard (entrada: &Vec<Token>) -> Vec<&Token>{
    let mut pilha: Vec<&Token> = Vec::new();
    let mut fila =  Vec::new();
    
    for i in entrada{
        match i {
            &Token::Num(x) =>{
                fila.push(i);
            }
            &Token::Operador(x) =>{
                while pilha.last() != Some(&&Token::Simbolo('(')) && !pilha.is_empty() && precedencia(pilha.last().unwrap(), i){
                    let _aux = pilha.pop().unwrap();
                    fila.push(_aux);
                }
                pilha.push(&i);
            }
            &Token::Simbolo(x) =>{
                if x == '('{
                    pilha.push(i);
                }
                else if x == ')'{
                    while pilha.last() != Some(&&Token::Simbolo('(')){
                        let _auxx = pilha.pop().unwrap();
                        fila.push(_auxx);
                    }
                    pilha.pop();
                }
            }
        }
    }
    while !pilha.is_empty() && pilha.last() != Some(&&Token::Simbolo('(')){
        let aux = pilha.pop().unwrap();
        fila.push(aux);
    }
fila
}

#[allow(unused)]
fn eval_step(entrada: &Arvore) -> Arvore{
   let mut saida = Arvore {direito: None, esquerda: None, valor: Token::Num(0)};
   let mut result: i64 = 0;
   let esq = entrada.esquerda.clone().unwrap().valor;
   let dir = entrada.direito.clone().unwrap().valor;
    match esq{
        Token::Num(y) =>{
            match dir{
                Token::Num(x) =>{
                    if entrada.valor == Token::Operador('+'){
                        result = y + x;
                        saida = Arvore {direito: None, esquerda:None, valor: Token::Num(result)};
                    }
                    if entrada.valor == Token::Operador('-'){
                        result = y - x;
                        saida = Arvore {direito: None, esquerda:None, valor: Token::Num(result)};
                    }
                    if entrada.valor == Token::Operador('*'){
                        result = y * x;
                        saida = Arvore {direito: None, esquerda:None, valor: Token::Num(result)};
                    }
                    if entrada.valor == Token::Operador('/'){
                        result = y / x;
                        saida = Arvore {direito: None, esquerda:None, valor: Token::Num(result)};
                    }
                }
                _=>{
                saida = Arvore {direito: Some(Box::new(eval_step(entrada.direito.clone().unwrap().as_ref()))), esquerda: Some(entrada.esquerda.clone().unwrap()), valor: entrada.valor.clone()};
                }
            }
        }
        _=>{
            saida = Arvore {direito: Some(entrada.direito.clone().unwrap()), esquerda: Some(Box::new(eval_step(entrada.esquerda.clone().unwrap().as_ref()))), valor: entrada.valor.clone()};
        }
    }
    saida
}

#[allow(unused)]
fn resolver(mut entrada: Arvore) -> Arvore{
    println!("{}", imprimir(&entrada));
    loop{
        match entrada.valor{
            Token::Num(x) =>{
                break;
            }
            _=>{
                entrada = eval_step(&entrada);
                println!("{}", imprimir(&entrada));
            }
        }
    }
    entrada
}

#[allow(unused)]
fn imprimir (entrada: &Arvore) -> String{
    let mut olha = String::from("");
    match entrada.valor{
        Token::Num(x) =>{
            olha.push_str(&x.to_string());
        }
        Token::Operador(x) =>{
            match &entrada.esquerda.clone().unwrap().valor{
                &Token::Num(y) =>{
                        olha.push_str("(");
                        olha.push_str(&imprimir(entrada.esquerda.clone().unwrap().as_ref()));
                        olha.push_str(&x.to_string());
                        olha.push_str(&imprimir(entrada.direito.clone().unwrap().as_ref()));
                        olha.push_str(")");
                }   _=>{
                        olha.push_str(&imprimir(entrada.esquerda.clone().unwrap().as_ref()));
                        olha.push_str(&x.to_string());
                        olha.push_str(&imprimir(entrada.direito.clone().unwrap().as_ref()));
                }
           
            }
        }_=>{
            println!("Não esperado");
        }
    }
    olha
}

fn main(){
    resolver(cria_arvore(shunting_yard(&lex("31  * (4 + 10)")))[0].clone());
    println!("");
    resolver(cria_arvore(shunting_yard(&lex("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)")))[0].clone());
    println!("");
    resolver(cria_arvore(shunting_yard(&lex("4 + 18 / (9 - 3)")))[0].clone());
    println!("");
    resolver(cria_arvore(shunting_yard(&lex("(15 * 5) + (130 / 23)")))[0].clone());
    println!("");
    resolver(cria_arvore(shunting_yard(&lex("-71 * (-76 * 91 * (10 - 5 - -82) - -79)")))[0].clone());
    println!("");
    resolver(cria_arvore(shunting_yard(&lex("(54 - -8 - -35 + -68 - -90) * -39 + -43 + -91 * -30")))[0].clone());
    println!("");
    
}

#[test]

fn test_lexer(){
    assert_eq!(lex("25 + 15"), [Token::Num(25), Token::Operador('+'), Token::Num(15)]);
    assert_eq!(lex("(13 - 1)"), [Token::Simbolo('('), Token::Num(13), Token::Operador('-'), Token::Num(1), Token::Simbolo(')')]);
    assert_eq!(lex("((21 / 10) * -1999)"), [Token::Simbolo('('), Token::Simbolo('('), Token::Num(21), Token::Operador('/'), Token::Num(10), Token::Simbolo(')'), Token::Operador('*'), Token::Num(-1999), Token::Simbolo(')') ]);
    assert_eq!(lex("(15 * 5) + (130 / 23)"), [Token::Simbolo('('), Token::Num(15), Token::Operador('*'), Token::Num(5), Token::Simbolo(')'), Token::Operador('+'), Token::Simbolo('('), Token::Num(130), Token::Operador('/'), Token::Num(23), Token::Simbolo(')')]);
    assert_eq!(lex("178 - -123 + 125 - 12 - 35 + -2"), [Token::Num(178), Token::Operador('-'), Token::Num(-123), Token::Operador('+'), Token::Num(125), Token::Operador('-'), Token::Num(12), Token::Operador('-'), Token::Num(35), Token::Operador('+'), Token::Num(-2)])
}

#[test]

fn test_shunting(){
    assert_eq!(shunting_yard(&lex("25 + 15")), [&Token::Num(25), &Token::Num(15),&Token::Operador('+')]);
    assert_eq!(shunting_yard(&lex("13 - 1")), [&Token::Num(13), &Token::Num(1), &Token::Operador('-')]);
    assert_eq!(shunting_yard(&lex("((21 / 10) * -1999)")), [&Token::Num(21), &Token::Num(10), &Token::Operador('/'), &Token::Num(-1999), &Token::Operador('*')]);
    assert_eq!(shunting_yard(&lex("(15 * 5) + (130 / 23)")), [&Token::Num(15), &Token::Num(5), &Token::Operador('*'), &Token::Num(130), &Token::Num(23), &Token::Operador('/'), &Token::Operador('+')]);
    assert_eq!(shunting_yard(&lex("4 + 18 / (9 - 3)")), [&Token::Num(4), &Token::Num(18), &Token::Num(9), &Token::Num(3), &Token::Operador('-'), &Token::Operador('/'), &Token::Operador('+')])
}

#[test]

fn test_resolver(){
    assert_eq!(resolver(cria_arvore(shunting_yard(&lex("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)")))[0].clone()).valor, Token::Num(-3357342660));
    assert_eq!(resolver(cria_arvore(shunting_yard(&lex("31  * (4 + 10)")))[0].clone()).valor, Token::Num(434));
    assert_eq!(resolver(cria_arvore(shunting_yard(&lex("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)")))[0].clone()).valor, Token::Num(135290));
    assert_eq!(resolver(cria_arvore(shunting_yard(&lex("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61")))[0].clone()).valor, Token::Num(-104777));
    assert_eq!(resolver(cria_arvore(shunting_yard(&lex("-71 * (-76 * 91 * (10 - 5 - -82) - -79)")))[0].clone()).valor, Token::Num(42714523));
    
}