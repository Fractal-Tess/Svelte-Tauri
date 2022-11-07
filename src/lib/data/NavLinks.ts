import type { NavLink } from '$types';

export const navLinks: NavLink[] = [
  {
    content: {
      text: 'Home'
    },
    target: {
      href: '/',
      newTab: false
    }
  },
  {
    content: {
      text: 'Call-Tauri'
    },
    target: {
      href: '/call-tauri',
      newTab: false
    }
  },
  {
    content: {
      text: 'Hash-string'
    },
    target: {
      href: '/hash-string',
      newTab: false
    }
  },

  {
    content: {
      text: 'Versions'
    },
    target: {
      href: '/versions',
      newTab: false
    }
  }
];
