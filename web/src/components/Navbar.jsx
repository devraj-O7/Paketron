import React from 'react';
import { Package } from 'lucide-react';
import { DownloadButton } from './ui/DownloadButton';

export function Navbar() {
  return (
    <nav className="w-full py-4 px-6 flex justify-between items-center bg-slate-900/50 backdrop-blur-md border-b border-slate-800 fixed top-0 z-50">
      <div className="flex items-center gap-2">
        <Package className="w-8 h-8 text-blue-500" />
        <span className="text-xl font-bold bg-gradient-to-r from-blue-400 to-cyan-400 bg-clip-text text-transparent">
          Paketron
        </span>
      </div>
      <div className="flex gap-6 text-sm font-medium text-slate-300">
        <a href="#" className="hover:text-white transition-colors">Features</a>
        <a href="#" className="hover:text-white transition-colors">Packages</a>
        <a href="#" className="hover:text-white transition-colors">Docs</a>
      </div>
      <DownloadButton className="text-sm px-4 py-2" />
    </nav>
  );
}
