<script setup lang="ts">
import { ref } from "vue";

const greetMsg = ref("");
const name = ref("");


</script>

<template>
  <form class="row" >
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
