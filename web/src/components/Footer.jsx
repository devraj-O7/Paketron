import React from 'react';

export function Footer() {
  return (
    <footer className="w-full py-8 px-6 bg-slate-950 border-t border-slate-900 mt-auto">
      <div className="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-4">
        <p className="text-slate-500 text-sm">
          © 2025 Paketron. Open Source.
        </p>
        <div className="flex gap-4 text-slate-500">
          <a href="#" className="hover:text-white transition-colors">GitHub</a>
          <a href="#" className="hover:text-white transition-colors">Twitter</a>
        </div>
      </div>
    </footer>
  );
}
