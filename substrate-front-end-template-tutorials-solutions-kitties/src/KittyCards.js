import React from 'react'
import {
  Button,
  Card,
  Grid,
  Message,
  Modal,
  Form,
  Label,
  Tab
} from 'semantic-ui-react'

//import KittyAvatar from './KittyAvatar'
import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

// --- Transfer Modal ---

const TransferModal = props => {
  const { domain, setStatus } = props
  const [open, setOpen] = React.useState(false)
  const [formValue, setFormValue] = React.useState({})

  const formChange = key => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value })
  }

  const confirmAndClose = unsub => {
    setOpen(false)
    if (unsub && typeof unsub === 'function') unsub()
  }

  return (
    <Modal
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
      open={open}
      trigger={
        <Button basic color="blue">
          Transfer
        </Button>
      }
    >
      <Modal.Header>Kitty Transfer</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Input fluid label="Domain ID" readOnly value={domain.id} />
          <Form.Input
            fluid
            label="Receiver"
            placeholder="Receiver Address"
            onChange={formChange('target')}
          />
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button basic color="grey" onClick={() => setOpen(false)}>
          Cancel
        </Button>
        <TxButton
          label="Transfer"
          type="SIGNED-TX"
          setStatus={setStatus}
          onClick={confirmAndClose}
          attrs={{
            palletRpc: 'substrateKitties',
            callable: 'transfer',
            inputParams: [formValue.target, domain.id],
            paramFields: [true, true],
          }}
        />
      </Modal.Actions>
    </Modal>
  )
}

// --- Set Price ---

const SetPrice = props => {
  const { domain, setStatus } = props
  const [open, setOpen] = React.useState(false)
  const [formValue, setFormValue] = React.useState({})

  const formChange = key => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value })
  }

  const confirmAndClose = unsub => {
    setOpen(false)
    if (unsub && typeof unsub === 'function') unsub()
  }

  return (
    <Modal
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
      open={open}
      trigger={
        <Button basic color="blue">
          Set Price
        </Button>
      }
    >
      <Modal.Header>Set Kitty Price</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Input fluid label="Domain ID" readOnly value={domain.id} />
          <Form.Input
            fluid
            label="Price"
            placeholder="Enter Price"
            onChange={formChange('target')}
          />
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button basic color="grey" onClick={() => setOpen(false)}>
          Cancel
        </Button>
        <TxButton
          label="Set Price"
          type="SIGNED-TX"
          setStatus={setStatus}
          onClick={confirmAndClose}
          attrs={{
            palletRpc: 'substrateKitties',
            callable: 'setPrice',
            inputParams: [domain.id, formValue.target],
            paramFields: [true, true],
          }}
        />
      </Modal.Actions>
    </Modal>
  )
}
// --- Set Wallet ---

const SetWallet = props => {
  const { domain, setStatus } = props
  const [open, setOpen] = React.useState(false)
  const [formValue, setFormValue] = React.useState({})

  const formChange = key => (ev, el) => {
    setFormValue({ ...formValue, [key]: el.value })
  }

  const confirmAndClose = unsub => {
    setOpen(false)
    if (unsub && typeof unsub === 'function') unsub()
  }

  return (
    <Modal
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
      open={open}
      trigger={
        <Button basic color="blue">
          Set Wallet
        </Button>
      }
    >
      <Modal.Header>Set Domain Wallet</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Input fluid label="Domain ID" readOnly value={domain.id} />
          <Form.Input
            fluid
            label="Wallet"
            placeholder="Enter Wallet"
            onChange={formChange('target')}
          />
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button basic color="grey" onClick={() => setOpen(false)}>
          Cancel
        </Button>
        <TxButton
          label="Set Wallet"
          type="SIGNED-TX"
          setStatus={setStatus}
          onClick={confirmAndClose}
          attrs={{
            palletRpc: 'substrateKitties',
            callable: 'setWallet',
            inputParams: [domain.id, formValue.target],
            paramFields: [true, true],
          }}
        />
      </Modal.Actions>
    </Modal>
  )
}

// --- Buy Kitty ---

const BuyKitty = props => {
  const { domain, setStatus } = props
  const [open, setOpen] = React.useState(false)

  const confirmAndClose = unsub => {
    setOpen(false)
    if (unsub && typeof unsub === 'function') unsub()
  }

  if (!domain.price) {
    return <></>
  }

  return (
    <Modal
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
      open={open}
      trigger={
        <Button basic color="green">
          Buy Domain
        </Button>
      }
    >
      <Modal.Header>Buy Domain</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Input fluid label="Domain ID" readOnly value={domain.id} />
          <Form.Input fluid label="Price" readOnly value={domain.price} />
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button basic color="grey" onClick={() => setOpen(false)}>
          Cancel
        </Button>
        <TxButton
          label="Buy Domain"
          type="SIGNED-TX"
          setStatus={setStatus}
          onClick={confirmAndClose}
          attrs={{
            palletRpc: 'substrateKitties',
            callable: 'buyDomain',
            inputParams: [domain.id, domain.price],
            paramFields: [true],
          }}
        />
      </Modal.Actions>
    </Modal>
  )
}

// --- About Kitty Card ---

const KittyCard = props => {
  const { domain1, setStatus } = props
  const { domain = null, owner = null, price = null, wallet = null} = domain1
  const displayDna = domain
  const { currentAccount } = useSubstrateState()
  const isSelf = currentAccount.address === domain1.owner

  return (
    <Card>
      {isSelf && (
        <Label as="a" floating color="teal">
          Mine
        </Label>
      )}
      {/* <KittyAvatar domain={displayDna} /> */}
      <Card.Content>
        <Card.Meta style={{ fontSize: '.9em', overflowWrap: 'break-word' }}>
          DOMAIN: <span style={{ fontSize: '2em', fontWeight: '900', color: 'green' }}> {displayDna} </span>
        </Card.Meta>
        <Card.Description>
          <p style={{ overflowWrap: 'break-word' }}>Owner: {owner}</p>
          {owner === currentAccount.address ? (
          <>
            <p style={{ overflowWrap: 'break-word' }}>Wallet: {wallet}</p>
          </>
        ) : (
          <>
          </>
        )}
          <p style={{ overflowWrap: 'break-word' }}>
            Price: {price || 'Not For Sale'}
          </p>
        </Card.Description>
      </Card.Content>
      <Card.Content extra style={{ textAlign: 'center' }}>
        {owner === currentAccount.address ? (
          <>
            <SetPrice domain={domain1} setStatus={setStatus} />
            <SetWallet domain={domain1} setStatus={setStatus} />
            <TransferModal domain={domain1} setStatus={setStatus} />
          </>
        ) : (
          <>
            <BuyKitty domain={domain1} setStatus={setStatus} />
          </>
        )}
      </Card.Content>
    </Card>
  )
}

const KittyCards = props => {
  const { currentAccount } = useSubstrateState()
  const { domains, setStatus } = props
  var mydomains = domains.reduce((mydomains, domain) => {
    if (domain.owner === currentAccount.address) {
      mydomains.push(domain);
    }
    return mydomains;
  }, []);
  var onsaledomains = domains.reduce((onsaledomains, domain) => {
    if (domain.price !== null) {
      onsaledomains.push(domain);
    }
    return onsaledomains;
  }, []);
  if (domains.length === 0) {
    return (
      <Message info>
        <Message.Header>
          No Kitty found here... Create one now!&nbsp;
          <span role="img" aria-label="point-down">
            👇
          </span>
        </Message.Header>
      </Message>
    )
  }
  const panes = [
    {
      menuItem: 'ALL',
      render: () =>
        <Tab.Pane attached={false}>
          <Grid columns={3}>
          {domains.map((domain1, i) => (
            <Grid.Column key={`domain-${i}`}>
              <KittyCard domain1={domain1} setStatus={setStatus} />
            </Grid.Column>
          ))}
          </Grid>
        </Tab.Pane>,
    },
    {
      menuItem: 'MY DOMAIN',
      render: () => 
        <Tab.Pane attached={false}>
          <Grid columns={3}>
          {mydomains.map((domain1, i) => (
            <Grid.Column key={`domain-${i}`}>
              <KittyCard domain1={domain1} setStatus={setStatus} />
            </Grid.Column>
          ))}
          </Grid>
        </Tab.Pane>,
    },
    {
      menuItem: 'DOMAIN SALE',
      render: () => 
      <Tab.Pane attached={false}>
        <Grid columns={3}>
        {onsaledomains.map((domain1, i) => (
          <Grid.Column key={`domain-${i}`}>
            <KittyCard domain1={domain1} setStatus={setStatus} />
          </Grid.Column>
        ))}
        </Grid>
      </Tab.Pane>,
    },
  ]
  
  return (
      <Tab menu={{ pointing: true }} panes={panes} />
  )
}

export default KittyCards
