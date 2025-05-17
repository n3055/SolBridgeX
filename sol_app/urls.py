from django.urls import path
from . import views
# from django.urls import path
urlpatterns = [
    path('',views.index,name='index'),
    path('dashboard/',views.dashboard,name='dashboard'),
    path('test/',views.test,name='test'),
]
