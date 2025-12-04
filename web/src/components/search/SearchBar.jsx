import React from 'react';
import { Search } from 'lucide-react';
import { Input } from '../ui/Input';

export function SearchBar({ value, onChange, className = '' }) {
  return (
    <div className={`relative max-w-2xl mx-auto ${className}`}>
      <Input
        icon={<Search className="w-5 h-5" />}
        placeholder="Search packages (e.g., vscode, rust, 7zip)..."
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="w-full bg-slate-800/80 border-slate-700 focus:border-blue-500 text-lg py-4 pl-14 shadow-xl shadow-blue-900/5"
      />
    </div>
  );
}
