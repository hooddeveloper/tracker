<template>
    <div id="app">
        <router-view/>
    </div>
</template>

<style>
    @import url('https://fonts.googleapis.com/css?family=Roboto:100,100i,200,200i,300,300i,400,400i,500,500i,600,600i,700,700i,800,800i,900,900i');

    #app {
        font-family: 'Roboto', sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
    }

    body, html {
        margin: 0;
        padding: 0;
    }

    .native {
        user-select: none;
    }

</style>
