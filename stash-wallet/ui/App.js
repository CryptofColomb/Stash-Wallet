import React from 'react';
import { LanguageProvider } from './components/LanguageSwitcher';
import Wallet from './pages/Wallet';
import Swap from './pages/Swap';
import Fiat from './pages/Fiat';
import CrossChain from './pages/CrossChain';
import Settings from './pages/Settings';

function App() {
  return (
    <LanguageProvider>
      <div className="app">
        <Settings />
        <Wallet />
        <Swap />
        <Fiat />
        <CrossChain />
      </div>
    </LanguageProvider>
  );
}

export default App;