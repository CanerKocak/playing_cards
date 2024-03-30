<script>
  import "../index.scss";
  import { backend } from "$lib/canisters";

  let greeting = "";

  function onSubmit(event) {
    const name = event.target.name.value;
    backend.greet(name).then((response) => {
      greeting = response;
    });
    return false;
  }
</script>

<main>
  <img src="/logo2.svg" alt="DFINITY logo" />
  <h1>Welcome to Playing Cards Terminal</h1>
</main>
