import React, { useEffect, useState } from 'react'
import { Form, Grid, Input, Label } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

// Construct a Kitty ID from storage key
const convertToKittyHash = entry =>
  `0x${entry[0].toJSON().slice(-64)}`;

// Construct a Kitty object
const constructDomain = (hash, {
  domain,
  price,
  wallet,
  owner,
  date_created
}) => ({
  id: hash,
  domain: domain.toHuman(),
  price: price.toJSON(),
  wallet: wallet.toHuman(),
  owner: owner.toJSON(),
  date_created: owner.toJSON()
});

export default function Kitties(props) {
  const { api, keyring } = useSubstrateState()
  const [domainHashes, setDomainHashes] = useState([])
  const [domains, setDomains] = useState([])
  const [status, setStatus] = useState('')

  const [formValue, setFormValue] = React.useState({})

  const formChange = key => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value+'.dot' })
  }
  const subscribeCount = () => {
    let unsub = null

    const asyncFetch = async () => {
      unsub = await api.query.substrateKitties.domainCnt(async cnt => {
        // Fetch all Kitty objects using entries()
        const entries = await api.query.substrateKitties.domains.entries();
        // Retrieve only the Kitty ID and set to state
        const hashes = entries.map(convertToKittyHash);
        setDomainHashes(hashes);
      });
    };


    asyncFetch()

    return () => {
      unsub && unsub()
    }
  }

    const subscribeKitties = () => {
      let unsub = null

      const asyncFetch = async () => {
        // Get Kitty objects from storage
        unsub = await api.query.substrateKitties.domains.multi(domainHashes, domains => {
          // Create an array of Kitty objects from `constructKitty`
          const kittyArr = domains.map((domain, ind) =>
            constructDomain(domainHashes[ind], domain.value)
          );
          // Set the array of Kitty objects to state
          setDomains(kittyArr)
        });
      };
    asyncFetch()

    return () => {
      unsub && unsub()
    }
  }

  useEffect(subscribeCount, [api, keyring])
  useEffect(subscribeKitties, [api, keyring, domainHashes])

  return (
    <Grid.Column width={16}>
      <h1>Domains</h1>
      <KittyCards domains={domains} setStatus={setStatus} />
      <Form style={{ margin: '1em 0' }}>
        <Form.Field style={{ textAlign: 'center' }}>
          <Input labelPosition='right' type='text'  
              fluid
              label="Domain"
              placeholder="Enter Domain"
              onChange={formChange('domain')}>
            <input
             />
            <Label>.dot</Label>
          </Input>
          <TxButton
            label="Create Domain"
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: 'substrateKitties',
              callable: 'createDomain',
              inputParams: [formValue.domain],
              paramFields: [true],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: 'break-word' }}>{status}</div>
    </Grid.Column>
  )
}
