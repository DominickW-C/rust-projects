import requests
from bs4 import BeautifulSoup


move_file = open("./src/pokeInfo/moveInfo.csv", "a")

# Gen 7 to avoid Z moves
move_resp = requests.get("https://bulbapedia.bulbagarden.net/wiki/List_of_moves_by_availability_in_Generation_VII")

if move_resp.status_code == 200:
    move_doc = move_resp.text
    move_soup = BeautifulSoup(move_doc, "html.parser")
    tables = move_soup.find_all("tbody")

    # move table
    move_table = tables[1]

    move_rows = move_table.find_all("tr")
    # first entry is not a move
    first_entry = 0
    for row in move_rows:
        if first_entry == 0: 
            first_entry += 1
            continue
        ret_string = ""
        move_data = row.find_all("td")
        print(move_data)
        ret_string += move_data[1].text.strip()
        ret_string += "," + move_data[2].text.strip()
        ret_string += "," + move_data[3].text.strip()
        ret_string += "," + move_data[4].text.strip()
        ret_string += "," + move_data[5].text.strip()
        ret_string += "," + move_data[6].text.strip().replace("%", "")
        ret_string += '\n'
        move_file.write(ret_string)
