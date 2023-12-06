from django.shortcuts import render

from pathlib import Path
from .data.Ship import Ship
import json

def home(request):
    return render(request=request, template_name='base.html')

def ship(request):
    
    this_dir = Path(__file__).parent
    json_file = this_dir / 'data/shipmanifest.json'
    parsed_json = json.loads(json_file.read_text())
    ship_dict = parsed_json['ships']['USA'][0]
    
    ship = Ship(**ship_dict)

    return render(request=request, template_name='Ship.html', context={'ship': ship})
# Create your views here.
