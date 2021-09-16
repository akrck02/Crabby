# GTDF_Crabby

CLI toolkit for GTD framework.

<div style="display:flex;">
    <img src="./meta/logo.png" style="width:90%; max-width:200px; max-height:200px; object-fit: contain; ">
</div>

## How to use crabby

#### 0. Parameters

​	Crabby is a CLI toolkit and gets parameters as output. All the main options have a short and large command. for exaple "crabby new" or "crabby n"	



#### 1. Create a new GTDF project

Create a new GTDF project in the linked version.

```bash
crabby n my_project
```



#### 2. Create a new view

Create a new directory with the typescrript view inside. 

```
crabby v my_view 
```



#### 3. Create a new component

If a view name is passed as argument, the component will belong to that view directory, otherwise, the component will be global. 

```bash
crabby c my_component [my_view]
```



#### 4. Create a new service

Create a new typescript data service

```bash
crabby s my_service
```



#### 5. Create a new test

Create a new test. You can pass the test type, default value will be unit testing.
