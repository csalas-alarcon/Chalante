<script>
    let name = 'Svelte';    
</script>

<h1>This was made with {name} </h1>
<input bind:value={name} placeholder="Enter your name" />