from csv import reader
from urllib import request, parse
import json


url = 'https://execut.fly.dev/register'


with open('data.csv', newline='') as file:
    rows = iter(reader(file))

    next(rows)

    for row in rows:
        [_, _, uuid, name, _, _, linkedin, study, degree, institution, graduation_year, company_slug, badges] = row

        # print(name)

        req = request.Request(url, method='POST')
        req.add_header('Content-Type', 'application/json')

        data = str(json.dumps({
          'id': uuid if uuid else None,
          'name': name if name else None,
          'linkedin': linkedin if linkedin else None,
          'study': study if study else None,
          'degree': degree if degree else None,
          'institution': institution if institution else None,
          'graduation_year': graduation_year if graduation_year else None,
          'company_slug': company_slug if company_slug else None,
          'badges': list(badges.split(','))
        }))

        data = data.encode('utf-8')

        res = request.urlopen(req, data=data)

        print(res.read().decode())
