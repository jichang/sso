import { TestBed } from '@angular/core/testing';

import { SecretModelService } from './secret-model.service';

describe('SecretModelService', () => {
  beforeEach(() => TestBed.configureTestingModule({}));

  it('should be created', () => {
    const service: SecretModelService = TestBed.get(SecretModelService);
    expect(service).toBeTruthy();
  });
});
