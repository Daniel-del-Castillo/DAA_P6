# %%
from plotnine import ggplot, aes, geom_point, geom_line
import pandas
import sys


data = pandas.read_csv('tcts/100it_total_it')
ggplot(data, aes(x='k', y='time', color='algorithm')) + \
    geom_point() + geom_line()

# %%
