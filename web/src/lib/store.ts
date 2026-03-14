"use client";

import { create } from "zustand";

interface AppStore {
  selectedNodeId: string | null;
  setSelectedNodeId: (id: string | null) => void;
  sidebarOpen: boolean;
  toggleSidebar: () => void;
}

export const useAppStore = create<AppStore>((set) => ({
  selectedNodeId: null,
  setSelectedNodeId: (id) => set({ selectedNodeId: id }),
  sidebarOpen: true,
  toggleSidebar: () => set((s) => ({ sidebarOpen: !s.sidebarOpen })),
}));
