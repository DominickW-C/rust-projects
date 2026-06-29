import requests
from bs4 import BeautifulSoup

allPokemon = open("./pokemonList.txt")
dex = open("./src/pokeInfo/pokedex.csv", "a")

while True:
    pokemon = allPokemon.readline().strip()
    if pokemon == "":
        break

    resp = requests.get(
        "https://bulbapedia.bulbagarden.net/wiki/" + pokemon + "_(Pokémon)")

    retString = ""

    if resp.status_code == 200:
        doc = resp.text
        soup = BeautifulSoup(doc, "html.parser")
        name = soup.find(id="firstHeading")
        retString += name.text.split(" ")[0]
        hp = soup.find(title="HP")
        retString += "," + hp.next_element.next_element.next_element.next_element.text
        check = 0
        for ii in soup.find_all(title="Stat"):
            # prevent mega evo stats
            if check == 5:
                break
            if ii.text == "Stat":
                continue
            retString += "," + ii.next_element.next_element.next_element.next_element.text
            check += 1
    else:
        print("error")

    retString += '\n'
    print("success")
    dex.write(retString)
