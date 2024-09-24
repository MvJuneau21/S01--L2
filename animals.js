class Animal {
  constructor(nome, idade, especie) {
    this.nome = nome;
    this.idade = idade;
    this.especie = especie;
  }

  printInfo() {
    console.log(`Nome: ${this.nome}, Idade: ${this.idade}, Espécie: ${this.especie}`);
  }
}

class Cachorro extends Animal {
  #raca;

  constructor(nome, idade, especie, raca) {
    super(nome, idade, especie);
    this.#raca = raca;
  }

  printInfo() {
    console.log(`Nome: ${this.nome}, Idade: ${this.idade}, Espécie: ${this.especie}, Raça: ${this.#raca}`);
  }
}

class Gato extends Animal {
  constructor(nome, idade, especie, cores) {
    super(nome, idade, especie);
    this.cores = cores;
  }

  printInfo() {
    console.log(`Nome: ${this.nome}, Idade: ${this.idade}, Espécie: ${this.especie}, Cores: ${this.cores.join(', ')}`);
  }
}

const animal = new Animal("Tigre", 3, "Felino");
const cachorro = new Cachorro("Costela", 7, "Canino", "Dachshund");
const gato = new Gato("Eevee", 1, "Felino", ["Laranja", "Branco"]);

animal.printInfo();
cachorro.printInfo();
gato.printInfo();
