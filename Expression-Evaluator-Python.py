class Fila():
    def __init__(self):
        self.fila = []
    def inserir (self, x):
        self.fila.append(x)
    def remover (self):
        del self.fila[0]

class Pilha():
    def __init__(self):
        self.pilha = []
    def topo (self):
        if (self.pilha == []):
            return []
        return self.pilha[len(self.pilha) - 1]
    def empilha (self, x):
        self.pilha.append(x)
    def desempilha (self):
        return self.pilha.pop()
        
class Node:
    def __init__(self, value, left, right):
        self.left = left
        self.data = value
        self.right = right

def cria_arvore (Polonesa):
    stack = Pilha ()
    while(Polonesa != []):
        if (isinstance(Polonesa[0], int) and stack.topo != []):
            stack.empilha(Node(Polonesa[0], None, None))
        elif(isinstance(Polonesa[0], str)):
            b = stack.desempilha()
            a = stack.desempilha()
            Exe = Node(Polonesa[0], a, b)
            stack.empilha(Exe)
        Polonesa.remove(Polonesa[0])
    return stack.pilha[0]

def lexer (exp):
    tokens = []
    digitos = ""
    for x in exp:
        if(x == " " or x ==")"):
            if (len(digitos) != 0):
                if(digitos == "-"):
                    tokens.append(digitos)
                    digitos = ""
                else:
                    tokens.append(int(digitos))
                    digitos = ""
        if(x.isdecimal()):
            digitos = digitos + x
            if (len(digitos) == 3):
                tokens.append(int(digitos))
                digitos = ""
        if( x == "*" or x == "+" or x == "/" or x =="(" or x ==")"):
            tokens.append(x)
        
        if( x == "-"):
            digitos = digitos + x
    return tokens

def precedencia (x, y):
    if isinstance(x, int):
        return True
    if isinstance(y, int):
        return False
    else:
        tabela = {'-' : 0, '*' : 1, '/' : 1, '+' : 0}
        return tabela[str(x)] >= tabela[str(y)]
    

def Shunting_yard (Tokens):
    fila = Fila()
    pilha = Pilha()
    while(Tokens != []):
        Token = Tokens[0]
        if (isinstance(Token,int)):
            fila.inserir(int(Token))
        elif (Token == "+" or Token == "-" or Token == "/" or Token == "*"):
            while ((pilha.topo() != '(') and (pilha.topo() != []) and (precedencia(pilha.topo(), Token))):
                fila.inserir(pilha.desempilha())
            pilha.empilha(Token)
        elif (Token == "("):
            pilha.empilha(Token)
        elif (Token == ")"):
            while(pilha.topo() != "("):
                fila.inserir(pilha.desempilha())
            pilha.desempilha()
        Tokens.remove(Tokens[0])
    while(pilha.pilha != []):
         fila.inserir(pilha.desempilha())
    return fila.fila

def Eval_step (Node):
    if (isinstance(Node.left.data, int) and isinstance(Node.right.data, int)):
        Node.data = int(eval("{0}{1}{2}".format(Node.left.data, Node.data, Node.right.data)))
        Node.left = None
        Node.right = None
    else:
        if(isinstance(Node.left.data, str)):
            Eval_step(Node.left)
        else:
            Eval_step(Node.right)
    return Node

def Resolver (Node):
    imprimir_final(imprimir(Node))
    print('\n')
    while (not isinstance(Node.data,int)):
        Node = Eval_step (Node)
        imprimir_final(imprimir(Node))
        print('\n')
    return Node.data

def imprimir (Node):
    lista = []
    if (isinstance(Node.data, int)):
        lista.append(Node.data)
    else:
        if (precedencia(Node.data, Node.left.data)):
            lista.append("(")
        lista.extend(imprimir (Node.left))
        if (precedencia(Node.data, Node.left.data)):
            lista.append(")")
        lista.append(Node.data)
        if (precedencia(Node.data, Node.right.data)):
            lista.append("(")
        lista.extend(imprimir (Node.right))
        if (precedencia(Node.data, Node.right.data)):
            lista.append(")")
    return lista

def imprimir_final(lista):
    for i in lista: 
        print(i, end="")

assert lexer("31  * (4 + 10)") == [31, "*", "(", 4, "+", 10, ")"]
assert lexer("25 + 38 + 88 + (-6 - -73) * (-83 + (53 + 97) * 14)") == [25, "+", 38, "+", 88, "+", "(", -6, "-", -73, ")", "*", "(", -83, "+", "(", 53, "+", 97, ")", "*", 14, ")"]
assert lexer("(10 / 3 + 23) * (1 - 4)") == ["(", 10, "/", 3, "+", 23, ")", "*", "(", 1, "-", 4, ")"]
assert lexer("-71 * (-76 * 91 * (10 - 5 - -82) - -79)") == [-71, "*","(", -76,"*", 91, "*", "(", 10, "-", 5, "-", -82, ")", "-", -79, ")"]
assert lexer("4 + 18 / (9 - 3)") == [4, "+", 18, "/", "(", 9, "-", 3, ")"]
assert lexer("(-72 - 50 * -74 + -45) * 92 * 21 * 5 * (-13 - 66 - 18)") == ["(", -72, "-", 50, "*", -74, "+", -45, ")", "*", 92, "*", 21, "*", 5,"*", "(", -13, "-", 66, "-", 18, ")"]
assert lexer("55 * 48 * -44 - -32 + 1 * -80 * -94 - 74 * -53 + -30 + -61") == [55, "*", 48, "*", -44, "-", -32, "+", 1, "*", -80, "*", -94, "-", 74, "*", -53, "+", -30, "+", -61]

assert Shunting_yard([25, "+", 38, "+", 88, "+", "(", -6, "-", -73, ")", "*", "(", -83, "+", "(", 53, "+", 97, ")", "*", 14, ")"]) == [25, 38, "+", 88,"+", -6, -73, "-", -83, 53, 97, "+", 14, "*", "+", "*", "+"]
assert Shunting_yard([-71, "*","(", -76,"*", 91, "*", "(", 10, "-", 5, "-", -82, ")", "-", -79, ")"]) == [-71, -76, 91, "*", 10, 5, "-", -82, "-", "*", -79, "-", "*"]
assert Shunting_yard([4, "+", 18, "/", "(", 9, "-", 3, ")"]) == [4, 18, 9, 3, "-", "/", "+"]
assert Shunting_yard([31, "*", "(", 4, "+", 10, ")"]) == [31, 4, 10, "+", "*"]
assert Shunting_yard(["(", 10, "/", 3, "+", 23, ")", "*", "(", 1, "-", 4, ")"]) == [10, 3, "/", 23, "+", 1, 4, "-", "*"]
assert Shunting_yard(["(", -72, "-", 50, "*", -74, "+", -45, ")", "*", 92, "*", 21, "*", 5,"*", "(", -13, "-", 66, "-", 18, ")"]) == [-72, 50, -74, "*", "-", -45, "+", 92, "*", 21, "*", 5, "*", -13, 66, "-", 18, "-", "*"]
assert Shunting_yard([55, "*", 48, "*", -44, "-", -32, "+", 1, "*", -80, "*", -94, "-", 74, "*", -53, "+", -30, "+", -61]) == [55, 48, "*", -44, "*", -32, "-", 1, -80, "*", -94, "*", "+", 74, -53, "*", "-", -30, "+", -61, "+"]

assert Resolver(cria_arvore([-71, -76, 91, "*", 10, 5, "-", -82, "-", "*", -79, "-", "*"])) == 42714523
assert Resolver(cria_arvore([4, 18, 9, 3, "-", "/", "+"])) == 7
assert Resolver(cria_arvore([10, 3, "/", 23, "+", 1, 4, "-", "*"])) == -78
assert Resolver(cria_arvore([25, 38, "+", 88,"+", -6, -73, "-", -83, 53, 97, "+", 14, "*", "+", "*", "+"])) == 135290
assert Resolver(cria_arvore([-72, 50, -74, "*", "-", -45, "+", 92, "*", 21, "*", 5, "*", -13, 66, "-", 18, "-", "*"])) == -3357342660
assert Resolver(cria_arvore([55, 48, "*", -44, "*", -32, "-", 1, -80, "*", -94, "*", "+", 74, -53, "*", "-", -30, "+", -61, "+"])) == -104777
assert Resolver(cria_arvore([31, 4, 10, "+", "*"])) == 434