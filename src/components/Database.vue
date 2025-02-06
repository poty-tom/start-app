<template>
    <h2>Database</h2>
    <button @click="getTaurimon">
        やっちゃえたうりもん
    </button>
    <div class="table">
        <table border="1">
            <tr>
                <th>たうりもんID</th>
                <th>たうりもんVALUE</th>
            </tr>
            <tr v-for="(val, idx) in tmons" :key="idx">
                <td>{{ val.tauri_id }}</td>
                <td>{{ val.tauri_value }}</td>
            </tr>
        </table>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const tmons = ref([]);
const getTaurimon = async () => {
    let taurimons: [] = await invoke('get_taurimons', {});
    tmons.value = [...taurimons];
    console.log(tmons);
    console.log(taurimons);
}

</script>

<style>
table {
    align: center
}
</style>