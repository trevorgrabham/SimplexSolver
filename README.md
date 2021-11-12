## Simplex Solver

#### Proper Use
The simplex solver is currently meant for use with **maximization** Linear Programs in **standard form**.

#### Features

###### Variable Selection
The simplex solver currently works for standard simplex tableaus and can select variables based upon either **Bland's Rule** or the variable with the **most negative reduced cost**.
The two options can be chosen through the `variable selection type` parameter, using `"bland"` and `"standard"` respectively.

###### Standard Algorithms
The simplex solver can currently only solve simplex tableaus in the standard way, but will soon allow for the user to choose between `"standard"`, `"dual"`, and `"revised"` simplex method options as well. 

###### Big M Algorithms
The simplex solver can currently only solve simplex tableaus with artificial variables using the **Two Phase** simplex method, but future updates will provide support for **Detached Coefficient** method as well. 

###### Misc.
The simplex solver also allows the option of retrieving the **basis inverse**, but this is not supported for all of the algorithm options. When the action is unsupported, it prints an error message and returns nothing. 

There will also be supported added for retrieval of dual **variables** in future updates.