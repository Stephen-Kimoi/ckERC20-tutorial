import { useState } from 'react';
import { useAccount, useContractWrite, useWaitForTransaction } from 'wagmi'; 
import { MinterHelper as helperContractAddress } from '../contracts/contracts-address.json'; 
import { parseUnits } from 'ethers/lib/utils';
import abi from '../contracts/MinterHelper.json'
import './Header.css'
import { cketh_starter_backend } from '../../../../declarations/cketh_starter_backend';
import { toast, ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

function Header() {
  const { address, isConnected } = useAccount();
  const [amount, setAmount] = useState(0);
  const [canisterDepositAddress, setCanisterDepositAddress] = useState("");
  const [transactionHash, setTransactionHash] = useState("");
  const [verificationResult, setVerificationResult] = useState(null);
  const [isVerifying, setIsVerifying] = useState(false);
  const [verificationError, setVerificationError] = useState(null);

  const SepoliaUSDCAddress = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238"; 

  // Use a standard ERC20 ABI
  const erc20ABI = [
    {
      "constant": false,
      "inputs": [
        {
          "name": "_spender",
          "type": "address"
        },
        {
          "name": "_value",
          "type": "uint256"
        }
      ],
      "name": "approve",
      "outputs": [
        {
          "name": "",
          "type": "bool"
        }
      ],
      "type": "function"
    }
  ];

  // Function for getting the deposit address
  const depositAddress = async () => {
    const depositAddress = await cketh_starter_backend.canister_deposit_principal();
    setCanisterDepositAddress(depositAddress);
  };

  // Function for approving the helper contract to spend Sepolia USDC
  const { write: approve, isLoading: isApproveLoading } = useContractWrite({
    address: SepoliaUSDCAddress,
    abi: erc20ABI, 
    functionName: "approve",
    args: [helperContractAddress, amount],
    onSuccess(data) {
      toast.info("Approval successful. You can now proceed with the deposit.");
      console.log("Approval data is: ", data); 
    },
    onError(error) {
      toast.error("Approval failed");
      console.error(error);
    }
  });

  // Function for calling the "deposit" function in the helper contract
  const { write: deposit, data, isLoading: isDepositLoading } = useContractWrite({
    address: helperContractAddress,
    abi: abi,
    functionName: "deposit",
    args: [SepoliaUSDCAddress, amount, canisterDepositAddress],
    onSuccess(data) {
      toast.info("Depositing Sepolia USDC");
    },
    onError(error) {
      toast.error("Deposit failed");
      console.error(error);
    }
  });

  const { isLoading: isTxLoading } = useWaitForTransaction({
    hash: data?.hash,
    onSuccess() {
      toast.info("Verifying the transaction on-chain");
      verifyTransaction(data.hash);
    },
    onError(error) {
      toast.error("Transaction failed or rejected");
      console.error(error);
    }
  });

  // Function for verifying the transaction on-chain
  const verifyTransaction = async (hash) => {
    setIsVerifying(true);
    setVerificationError(null);

    try {
      const result = await cketh_starter_backend.verify_transaction(hash);
      setVerificationResult(result);
      toast.success("Transaction verified successfully");
    } catch (error) {
      setVerificationError("Verification failed. Please check the transaction hash and try again.");
      toast.error("Verification failed");
      console.error(error);
    } finally {
      setIsVerifying(false);
    }
  };

  const changeAmountHandler = (e) => {
    let amount = e.target.valueAsNumber;
    if (Number.isNaN(amount) || amount < 0) amount = 0;
    const usdcAmount = amount * Math.pow(10, 6); 
    setAmount(usdcAmount);
  };

  const changeAddressHandler = (e) => {
    setCanisterDepositAddress(e.target.value);
  };

  const changeTransactionHashHandler = (e) => {
    setTransactionHash(e.target.value);
  };

  return (
    <div className='container'>
      <ToastContainer />
      <h1 className='title'>ckSepoliaUSDC Tester</h1>
      
      <div className='wallet-info'>
        {isConnected ? (
          <p>Connected Wallet: <strong>{address}</strong></p>
        ) : (
          <p>No wallet connected</p>
        )}
      </div>

      <button onClick={depositAddress} className='button'>Get Deposit Address</button>

      <div className='form'>
        <input 
          type="text" 
          value={canisterDepositAddress} 
          onChange={changeAddressHandler} 
          placeholder="Canister Deposit Address" 
          disabled={isApproveLoading || isDepositLoading || isTxLoading}
          className='input'
        />
        <input 
          type="number" 
          onChange={changeAmountHandler} 
          placeholder="Amount" 
          disabled={isApproveLoading || isDepositLoading || isTxLoading}
          className='input'
        />
        <button onClick={() => approve()} disabled={isApproveLoading || isDepositLoading || isTxLoading} className='button'>
          {isApproveLoading ? 'Approving...' : 'Approve'}
        </button>
        <button onClick={() => deposit()} disabled={isApproveLoading || isDepositLoading || isTxLoading} className='button'>
          {isDepositLoading ? 'Processing...' : 'Deposit'}
        </button>
        {(isApproveLoading || isDepositLoading || isTxLoading) && <div className="loading-indicator">Loading...</div>}
      </div>

      <div className='form'>
        <input 
          type="text" 
          value={transactionHash} 
          onChange={changeTransactionHashHandler} 
          placeholder="Transaction Hash" 
          disabled={isVerifying}
          className='input'
        />
        <button onClick={() => verifyTransaction(transactionHash)} disabled={isVerifying} className='button'>
          {isVerifying ? 'Verifying...' : 'Verify Transaction'}
        </button>
        {isVerifying && <div className="loading-indicator">Verifying transaction...</div>}
      </div>

      {verificationResult && (
        <div className='verification-result'>
          <h2>Verification Result:</h2>
          <pre>{JSON.stringify(verificationResult, null, 2)}</pre>
        </div>
      )}

      {verificationError && (
        <div className='error-message'>
          <p>{verificationError}</p>
        </div>
      )}
    </div>
  );
}

export default Header;
