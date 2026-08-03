# Multiple Soups are probably not needed but they work

import requests
from bs4 import BeautifulSoup
import re

allPokemon = open("./pokemonList.txt")
dex = open("./src/pokeInfo/pokedex.csv", "a")
monMoves = open("./src/pokeInfo/movesByMon.csv", "a")

last_dex_num = 0
catch_index = 2
catch_resp = requests.get(
        "https://bulbapedia.bulbagarden.net/wiki/List_of_Pokémon_by_catch_rate")

if catch_resp.status_code == 200:
    catch_doc = catch_resp.text
    catch_soup = BeautifulSoup(catch_doc, "html.parser")
    table = catch_soup.find("table")
    table_soup = BeautifulSoup(table.prettify(), "html.parser")
    table_lines = table_soup.find_all("tr")


def catch_stat():
    global last_dex_num
    global catch_index
    data = table_lines[catch_index].find_all("td")
    while data[0].text.strip() == last_dex_num:
        catch_index += 1
        data = table_lines[catch_index].find_all("td")
    last_dex_num = data[0].text.strip()
    catch_index += 1
    return data[3].text.strip()


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

        # moves
        movesRet = pokemon
        moves = soup.find(id="By_leveling_up")
        while moves.name != "table":
            moves = moves.next
        movesRow = moves.find_all("tr")
        skip = 0
        for row in movesRow:
            moveInfo = row.find_all("td")
            if len(moveInfo) < 6:
                continue
            if skip == 0:
                skip = 1
                continue
            movesRet += "," + moveInfo[1].text.strip()
            movesRet += "," + moveInfo[0].text[2:].strip()
            movesRet += '\n'
            monMoves.write(movesRet)
            movesRet = pokemon

        # tm moves
        tmMoves = soup.find(id=re.compile("By_TM"))
        while tmMoves.name != "table":
            tmMoves = tmMoves.next
        tmMovesRow = tmMoves.find_all("tr")
        for row in tmMovesRow:
            tmMoveInfo = row.find_all("td")
            if len(tmMoveInfo) < 6:
                continue
            movesRet += "," + tmMoveInfo[2].text.strip()
            movesRet += "," + "-"
            movesRet += '\n'
            monMoves.write(movesRet)
            movesRet = pokemon

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
        ret_string += "," + catch_stat()
        ret_string += '\n'
        print(ret_string)
        dex.write(ret_string)
    else:
        print("fail")
