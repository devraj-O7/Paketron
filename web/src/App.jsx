import React from 'react';
import { Navbar } from './components/Navbar';
import { Footer } from './components/Footer';
import { Home } from './pages/Home';

function App() {
  return (
    <div className="min-h-screen bg-dark-bg text-white font-sans selection:bg-blue-500/30">
      <Navbar />
      <main>
        <Home />
      </main>
      <Footer />
    </div>
  );
}

export default App;
