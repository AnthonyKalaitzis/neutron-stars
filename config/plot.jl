using PlotlyJS: plot
using CSV
using DataFrames

eos = CSV.read("config/qmc700_with_crust.csv", DataFrame, header=false)
plot(eos, x=:Column1, y=:Column2, mode="lines")