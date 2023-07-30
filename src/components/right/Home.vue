<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";
// invoke('genpsd', { seed: 'test',length:16 })
//   // `invoke` 返回的是一个 Promise
//   .then((response) => console.log(response))
defineProps({
    hidden: String,
});
const password = ref(" ");
var change = function(e){
    console.log(e.target.value,"<<<");
invoke('genpsd', { seed: e.target.value,length:16 })
  // `invoke` 返回的是一个 Promise
//   .then((response) =>document.getElementById("password").setAttribute("value",response))
.then((response) => password.value = response)
}
</script>
<template>
    <form class="right-home" :hidden="hidden">
        <div class="maincontainer">
            <div class="show">
                <div class="drop"></div>
                <div class="drop"></div>
                <div class="drop"></div>
                <div class="drop"></div>
                <div class="ground"></div>
                <div class="ground"></div>
                <div class="ground"></div>
                <div class="ground"></div>
            </div>
            <div class="cup"></div>
            <p id="title">Password Generator</p>
            <div>
                <input id="seed" type="text" @change="change"/>
                <input id="password" type="text" :value="password" />
            </div>
        </div>
    </form>
</template>
<style scoped>
.right .right-home {
    background-color: #000;
    flex-grow: 1;
    justify-content: center;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
}
.drop{
    background-color: #fff;
    width: 30px;
    height: 45px;
    filter: blur(12px);
    border-radius: 50%;
    position: absolute;
    top:5px;
    /* left: 80px; */
}
.drop:nth-child(1){
    animation:dr 1s ease-in-out infinite;
}
.drop:nth-child(2){
    animation: dr 1s ease-in-out 0.3s infinite;
}
.drop:nth-child(3){
    animation: dr 1s ease-in-out 0.6s infinite;
}
.drop:nth-child(4){
    animation: dr 1s ease-in-out 0.9s infinite;
}
@keyframes dr{
    from{top:0px}
    to{top:120px}
}
.cup{
    width: 230px;
    height: 60px;
    position: relative;
    border-radius: 20px;
    top:-20px;
    left: 85px;
    background-color: #fff;
}
.show{
    height: 180px;
    padding-top: 110px;
    top: 40px;
    display: flex;
    position: relative;
    filter: contrast(50);
    background-color: #000;
    justify-content: center;
}
.ground{
    width: 56px;
    height: 56px;
    background-color: #fff;
    animation: ro 3s linear infinite;
    filter: blur(12px);
}
@keyframes ro {
    from{
        transform: rotate(0deg);
    }
    to{
        transform: rotate(360deg);
    }
}
#password {
    height: 30px;
    width: 200px;
    animation: seedInput 4s backwards;
}

#seed {
    animation: seedInput 2s backwards;
    height: 30px;
    width: 200px;
}

@keyframes seedInput {
    from {
        opacity: 0;
    }

    to {
        opacity: 1;
    }
}

#title {
    text-align: center;
    animation: fadeIn 2s forwards;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        font-size: 0;
        color: chartreuse;
    }

    to {
        opacity: 1;
        font-size: 25px;
        color: white;
    }
}</style>
