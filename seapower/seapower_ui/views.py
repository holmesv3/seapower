from django.shortcuts import render, HttpResponse

def home(request):
    return render(request=request, template_name='base.html')

# Create your views here.
