import App from './App.svelte';
import wasm from '../wasm/Cargo.toml';

const init = async () => {
  const phylogeneticTree = await wasm();

  const app = new App({
    target: document.body,
    props: {
      // https://svelte.dev/docs#Creating_a_component
      greet: phylogeneticTree.greet,
    },
  });
};

init();
