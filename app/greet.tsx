'use client'

import { GreetApi } from '@/lib/greet'

import { useEffect, useState } from 'react';

export default function Greet() {
  const [greeting, setGreeting] = useState('');

  useEffect(() => {
    GreetApi('Next.js')
      .then(result => setGreeting(result))
      .catch(console.error)
  }, [])

  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}
