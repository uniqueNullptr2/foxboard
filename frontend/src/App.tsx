import type { Component } from 'solid-js';
import { HopeProvider } from '@hope-ui/solid'
import logo from './logo.svg';
import styles from './App.module.css';
import { Project } from './Components/Projects';

const App: Component = () => {
  return (
    <HopeProvider>
      <Project></Project>
    </HopeProvider>
  );
};

export default App;
