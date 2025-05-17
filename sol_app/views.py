from django.shortcuts import render

# Create your views here.
def index(request):
    # Render the index.html template
    return render(request, 'index.html')

def dashboard(request):
    # Render the dashboard.html template
    return render(request, 'dashboard.html')

def test(request):
    # Render the test.html template
    return render(request, 'test.html')