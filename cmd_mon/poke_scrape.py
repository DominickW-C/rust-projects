import requests
from bs4 import BeautifulSoup
import re

allPokemon = open("./pokemonList.txt")
dex = open("./src/pokeInfo/pokedex.csv", "a")

while True:
    pokemon = allPokemon.readline().strip()
    if pokemon == "":
        break

    resp = requests.get(
        "https://bulbapedia.bulbagarden.net/wiki/" + pokemon + "_(Pokémon)")

    ret_string = ""

    if resp.status_code == 200:
        doc = resp.text
        soup = BeautifulSoup(doc, "html.parser")

        # name
        name = soup.find(id="firstHeading")
        name_list = name.text.split(" ")
        if name_list[1] != "(Pokémon)":
            ret_string += name_list[0] + " " + name_list[1]
        else:
            ret_string += name_list[0]

        # stats
        stat_table = soup.find(id="Base_stats")
        while stat_table.name != "table":
            stat_table = stat_table.next
        stat_table = stat_table.prettify()
        stat_soup = BeautifulSoup(stat_table, "html.parser")

        # hp
        hp = stat_soup.find(href=re.compile("/wiki/HP"))
        ret_string += "," + hp.next.next.next.next.next.next.next.text.strip()

        base_st = stat_soup.find_all(href=re.compile("/wiki/Stat#"))
        base_check = 0
        for stat in base_st:
            if base_check == 5:
                break
            ret_string += "," + stat.next.next.next.next.next.next.next.text.strip()
            base_check += 1
        ret_string += '\n'
        print(ret_string)
        dex.write(ret_string)
    else:
        print("fail")
