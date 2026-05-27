import fetch from 'cross-fetch';
import { getIntrospectionQuery, buildClientSchema, GraphQLSchema } from 'graphql';
import { filterSchema, pruneSchema } from '@graphql-tools/utils';

const clientMutations = [
  'addObjectGuests',
  'bulkCreateObjects',
  'confirmFileArtifactUpload',
  'createAgentTask',
  'createAnonymousUser',
  'createFileArtifactUploadTarget',
  'createFolder',
  'createGenericStringObject',
  'createManagedSecret',
  'createNotebook',
  'createWorkflow',
  'deleteConversation',
  'deleteManagedSecret',
  'updateManagedSecret',
  'deleteObject',
  'emptyTrash',
  'generateCommands',
  'generateDialogue',
  'generateMetadataForCommand',
  'generateUniqueUpgradePromoCode',
  'giveUpNotebookEditAccess',
  'grabNotebookEditAccess',
  'issueTaskIdentityToken',
  'leaveObject',
  'markAcceptedIntelligentAutosuggestion',
  'mintCustomToken',
  'moveObject',
  'provideNegativeFeedbackResponseForAiConversation',
  'purchaseAddonCredits',
  'recordObjectAction',
  'removeObjectGuest',
  'removeObjectLinkPermissions',
  'setObjectLinkPermissions',
  'setUserIsOnboarded',
  'shareBlock',
  'stripeBillingPortal',
  'transferGenericStringObjectOwner',
  'transferNotebookOwner',
  'transferWorkflowOwner',
  'trashObject',
  'unshareBlock',
  'untrashObject',
  'updateAgentTask',
  'updateFolder',
  'updateGenericStringObject',
  'updateNotebook',
  'updateObjectGuests',
  'updateUserSettings',
  'updateWorkflow',
  'updateWorkspaceSettings',
  'createSimpleIntegration',
];

const clientQueries = [
  'cloudObject',
  'freeAvailableModels',
  'harnessAuthSecrets',
  'listWarpDevImages',
  'pricingInfo',
  'managedSecrets',
  'updatedCloudObjects',
  'user',
  'userGithubInfo',
  'userRepoAuthStatus',
  'apiKeys',
  'getOAuthConnectTxStatus',
  'getCloudEnvironments',
  'simpleIntegrations',
  'getIntegrationsUsingEnvironment',
  'scheduledAgentHistory',
  'task',
  'taskGitCredentials',
  'taskSecrets',
  'listAIConversations',
  'suggestCloudEnvironmentImage'
];

const clientSubscriptions = ['warpDriveUpdates'];

function filterToClient(schema: GraphQLSchema): GraphQLSchema {
  const filtered = filterSchema({
    schema,
    rootFieldFilter: (operation, rootFieldName) => {
      if (operation === 'Query') {
        return clientQueries.includes(rootFieldName);
      } else if (operation === 'Mutation') {
        return clientMutations.includes(rootFieldName);
      } else if (operation === 'Subscription') {
        return clientSubscriptions.includes(rootFieldName);
      } else {
        console.error(`Unknown operation ${operation}.${rootFieldName}`);
        return true;
      }
    }
  });

  return pruneSchema(filtered);
}

module.exports = async (schemaUrl: string) => {
  // Adapted from https://the-guild.dev/graphql/codegen/docs/config-reference/schema-field#custom-schema-loader
  const response = await fetch(schemaUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ query: getIntrospectionQuery() }),
  });
  const data = await response.json();
  const schema = buildClientSchema(data.data);
  return filterToClient(schema);
};
