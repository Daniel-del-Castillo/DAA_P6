# %%
from plotnine import ggplot, aes, geom_point, geom_line, xlab, ylab
import pandas
import sys


# %%
data = pandas.read_csv('results/grasp_8')
ggplot(data, aes(x='iterations', y='time', color='algorithm')) + \
    geom_point() + geom_line() + \
    xlab('Iterations') + \
    ylab('Time (in ms)')

# %%

data = pandas.read_csv('results/grasp_8')
ggplot(data, aes(x='iterations', y='tct', color='algorithm')) + \
    geom_point() + geom_line() + \
    xlab('Iterations') + \
    ylab('TCT')

# %%
