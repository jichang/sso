import { TestBed, inject } from '@angular/core/testing';

import { AuthorizationModelService } from './authorization-model.service';

describe('AuthorizationModelService', () => {
  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [AuthorizationModelService]
    });
  });

  it('should be created', inject([AuthorizationModelService], (service: AuthorizationModelService) => {
    expect(service).toBeTruthy();
  }));
});
